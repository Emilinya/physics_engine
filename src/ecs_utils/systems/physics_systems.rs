use cgmath::InnerSpace;

use crate::ecs_utils::components::{ConnectionComponent, PhysicsComponent, PositionComponent, SpringForceComponent};
use crate::ecs_utils::systems::zip_filter_unwrap;

fn gravity_system(physics_components: &mut [Option<PhysicsComponent>]) {
    for physics_component in zip_filter_unwrap!(physics_components; as_mut) {
        physics_component.acceleration -= 9.81_f32 * cgmath::Vector2::unit_y();
    }
}

fn spring_system(
    spring_force_components: &Vec<Option<SpringForceComponent>>,
    connection_components: &Vec<Option<ConnectionComponent>>,
    position_components: &Vec<Option<PositionComponent>>,
    physics_components: &mut [Option<PhysicsComponent>]
) {
    let num_components = position_components.len();
    for (spring_force, connection) in zip_filter_unwrap!(spring_force_components; as_ref; 0, connection_components; as_ref; 1) {
        // check if connected entities exists
        if (connection.entity1 >= num_components) | (connection.entity2 >= num_components) {
            panic!(
                "Error when applying spring force: entity1 ({:?}) or entity2 ({:?}) does not exist! Number of components is {:?}",
                connection.entity1, connection.entity2, num_components
            );
        }
        let (pos1, pos2) = match (&position_components[connection.entity1], &position_components[connection.entity2]) {
            (Some(c1), Some(c2)) => (c1.position, c2.position),
            _ => {
                panic!(
                    "Error when applying spring force: entity1 ({:?}) or entity2 ({:?}) does not have a position!",
                    connection.entity1, connection.entity2
                );
            }
        };

        let between = pos2 - pos1;
        let length = between.magnitude();
        let unit = between / length;

        if let Some(phy1) = physics_components[connection.entity1].as_mut() {
            phy1.acceleration += spring_force.spring_constant * unit * (length - spring_force.equilibrium_length) / phy1.mass;
        }
        if let Some(phy2) = physics_components[connection.entity2].as_mut() {
            phy2.acceleration -= spring_force.spring_constant * unit * (length - spring_force.equilibrium_length) / phy2.mass;
        }
    }
}

fn acceleration_system(
    spring_force_components: &Vec<Option<SpringForceComponent>>,
    connection_components: &Vec<Option<ConnectionComponent>>,
    position_components: &Vec<Option<PositionComponent>>,
    physics_components: &mut [Option<PhysicsComponent>],
) {
    gravity_system(physics_components);
    spring_system(
        spring_force_components,
        connection_components,
        position_components,
        physics_components,
    );
}

pub fn energy_system(
    spring_force_components: &Vec<Option<SpringForceComponent>>,
    connection_components: &Vec<Option<ConnectionComponent>>,
    position_components: &Vec<Option<PositionComponent>>,
    physics_components: &Vec<Option<PhysicsComponent>>,
) -> f32 {
    let mut total_energy = 0_f32;
    
    // kinetic energy
    for physics_component in zip_filter_unwrap!(physics_components; as_ref) {
        total_energy += 0.5 * physics_component.mass * physics_component.velocity.magnitude2();
    }

    // gravitational potential (filter out entities without physics)
    for (position_component, _) in zip_filter_unwrap!(position_components; as_ref; 0, physics_components; as_ref; 1) {
        total_energy += position_component.position.y;
    }

    // spring potential
    for (spring_force_component, connection_component) in zip_filter_unwrap!(spring_force_components; as_ref; 0, connection_components; as_ref; 1) {
        let (pos1, pos2) = match (&position_components[connection_component.entity1], &position_components[connection_component.entity2]) {
            (Some(c1), Some(c2)) => (c1.position, c2.position),
            _ => {
                panic!(
                    "Error when calculating spring potential: entity1 ({:?}) or entity2 ({:?}) does not have a position!",
                    connection_component.entity1, connection_component.entity2
                );
            }
        };

        let elongation = (pos2 - pos1).magnitude() - spring_force_component.equilibrium_length;
        total_energy += 0.5 * spring_force_component.spring_constant * elongation.powi(2);
    }

    total_energy
}


fn physics_diff_system(
    position_components: &Vec<Option<PositionComponent>>,
    physics_components: &mut [Option<PhysicsComponent>],
) -> (Vec<cgmath::Vector2<f32>>, Vec<cgmath::Vector2<f32>>) {
    // calculate derivatives
    zip_filter_unwrap!(position_components; as_ref; 0, physics_components; as_mut; 1)
        .map(|(_, phys)| {
            let diff = (phys.velocity, phys.acceleration);
            phys.acceleration = cgmath::Vector2::new(0.0, 0.0);
            diff
        }).unzip()
}

fn physics_step(
    position_components: &mut [Option<PositionComponent>],
    physics_components: &mut [Option<PhysicsComponent>],
    position_derivatives: &[cgmath::Vector2<f32>],
    velocity_derivatives: &[cgmath::Vector2<f32>],
    dt: f32,
) {
    for (i, (position_components, physics_components)) in zip_filter_unwrap!(position_components; as_mut; 0, physics_components; as_mut; 1).enumerate() {
        position_components.position += position_derivatives[i] * dt;
        physics_components.velocity += velocity_derivatives[i] * dt;
    }
}

pub fn physics_system(
    spring_force_components: &Vec<Option<SpringForceComponent>>,
    connection_components: &Vec<Option<ConnectionComponent>>,
    position_components: &mut Vec<Option<PositionComponent>>,
    physics_components: &mut Vec<Option<PhysicsComponent>>,
    dt: &instant::Duration,
) {
    let mut position_copy = position_components.clone();
    let mut physics_copy = physics_components.clone();

    let dt_secs = dt.as_secs_f32();
    if dt_secs > 0.1 {
        log::error!("dt is large ({}), this could break physics", dt_secs);
    }

    // k1
    acceleration_system(
        spring_force_components,
        connection_components,
        &position_copy,
        &mut physics_copy,
    );
    let (k1p, k1v) = physics_diff_system(
        &position_copy,
        &mut physics_copy,
    );
    
    // k2
    physics_step(
        &mut position_copy,
        &mut physics_copy,
        &k1p,
        &k1v,
        dt_secs * 0.5
    );
    acceleration_system(
        spring_force_components,
        connection_components,
        &position_copy,
        &mut physics_copy,
    );
    let (k2p, k2v) = physics_diff_system(
        &position_copy,
        &mut physics_copy,
    );
    position_copy = position_components.clone();
    physics_copy = physics_components.clone();

    // k3
    physics_step(
        &mut position_copy,
        &mut physics_copy,
        &k2p,
        &k2v,
        dt_secs * 0.5
    );
    acceleration_system(
        spring_force_components,
        connection_components,
        &position_copy,
        &mut physics_copy,
    );
    let (k3p, k3v) = physics_diff_system(
        &position_copy,
        &mut physics_copy,
    );
    position_copy = position_components.clone();
    physics_copy = physics_components.clone();

    // k4
    physics_step(
        &mut position_copy,
        &mut physics_copy,
        &k3p,
        &k3v,
        dt_secs
    );
    acceleration_system(
        spring_force_components,
        connection_components,
        &position_copy,
        &mut physics_copy,
    );
    let (k4p, k4v) = physics_diff_system(
        &position_copy,
        &mut physics_copy,
    );

    for (i, (position_components, physics_components)) in zip_filter_unwrap!(position_components; as_mut; 0, physics_components; as_mut; 1).enumerate() {
        let dp = (k1p[i] + 2.0 * k2p[i] + 2.0 * k3p[i] + k4p[i]) / 6.0;
        let dv = (k1v[i] + 2.0 * k2v[i] + 2.0 * k3v[i] + k4v[i]) / 6.0;
        position_components.position += dp * dt_secs;
        physics_components.velocity += dv * dt_secs;
    }
}
