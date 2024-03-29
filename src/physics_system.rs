use bevy::math::{DVec2, Vec3};
use bevy::prelude::*;

use crate::integrators::{Integrator, Integrators};
use crate::{components::*, TotalEnergy};

pub struct PhysicsPlugin;

// g = π²
const GRAVITY: f64 = 9.81;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        Integrators::EulerChromer.build(app, (apply_gravity, apply_spring_force));
        app.add_systems(
            Update,
            (calculate_total_energy, update_transform, update_spring),
        );
    }
}

fn get_spring_connection_positions(
    connection: &Connection,
    position_query: &Query<&Position, Without<Spring>>,
) -> (DVec2, DVec2) {
    const SPRING_CONNECTION_ERROR: &str =
        "a connection is pointing to an entity without a position!";

    let pos1 = position_query
        .get(connection.entity1)
        .expect(SPRING_CONNECTION_ERROR)
        .0;
    let pos2 = position_query
        .get(connection.entity2)
        .expect(SPRING_CONNECTION_ERROR)
        .0;

    (pos1, pos2)
}

pub fn apply_gravity(mut query: Query<&mut PhysicsObject>) {
    for mut physics_component in query.iter_mut() {
        physics_component.acceleration -= GRAVITY * DVec2::Y;
    }
}

pub fn apply_spring_force(
    spring_query: Query<(&SpringForce, &Connection)>,
    position_query: Query<&Position, Without<Spring>>,
    mut physics_query: Query<&mut PhysicsObject>,
) {
    for (spring_force, connection) in spring_query.iter() {
        let (pos1, pos2) = get_spring_connection_positions(connection, &position_query);

        let between = pos2 - pos1;
        let length = between.length();
        let direction = between / length;

        if let Ok(mut phy1) = physics_query.get_mut(connection.entity1) {
            let mass = phy1.mass;
            phy1.acceleration += spring_force.spring_constant
                * direction
                * (length - spring_force.equilibrium_length)
                / mass;
        }
        if let Ok(mut phy2) = physics_query.get_mut(connection.entity2) {
            let mass = phy2.mass;
            phy2.acceleration -= spring_force.spring_constant
                * direction
                * (length - spring_force.equilibrium_length)
                / mass;
        }
    }
}

fn update_spring(
    mut spring_query: Query<(&Connection, &mut Position, &mut Size, &mut Rotation), With<Spring>>,
    position_query: Query<&Position, Without<Spring>>,
) {
    for (connection, mut position, mut size, mut rotation) in spring_query.iter_mut() {
        let (pos1, pos2) = get_spring_connection_positions(connection, &position_query);

        let midpoint = (pos1 + pos2) / 2.0;
        let between = pos1 - pos2;
        let length = between.length();

        position.0 = midpoint;
        rotation.0 = DVec2::angle_between(DVec2::X, between);
        size.width = length;
    }
}

fn update_transform(
    camera_query: Query<&Camera>,
    mut transform_query: Query<(&mut Transform, &Position, &Size, &Rotation)>,
) {
    let camera = camera_query.get_single().unwrap();
    let viewport_size = camera
        .logical_viewport_size()
        .expect("Can't get viewport size?!");
    let scale = viewport_size.x.min(viewport_size.y);

    for (mut transform, position, size, rotation) in transform_query.iter_mut() {
        let z = transform.translation.z;
        *transform = Transform {
            translation: Vec3::new(position.0.x as f32 * scale, position.0.y as f32 * scale, z),
            rotation: Quat::from_rotation_z(rotation.0 as f32),
            scale: Vec3::new(size.width as f32 * scale, size.height as f32 * scale, 1.0),
        };
    }
}

fn calculate_total_energy(
    timer: Res<Time>,
    mut total_energy_resource: ResMut<TotalEnergy>,
    spring_query: Query<(&SpringForce, &Connection)>,
    position_query: Query<&Position, Without<Spring>>,
    body_query: Query<(&Position, &PhysicsObject)>,
) {
    let mut total_energy = 0.0;

    for (position, physics_object) in body_query.iter() {
        // calculate kinetic energy
        total_energy += 0.5 * physics_object.mass * physics_object.velocity.length_squared();

        // calculate gravitational potential energy
        total_energy += physics_object.mass * GRAVITY * position.0.y;
    }

    // calculate spring force potential
    for (spring_force, connection) in spring_query.iter() {
        let (pos1, pos2) = get_spring_connection_positions(connection, &position_query);

        let elongation = (pos2 - pos1).length() - spring_force.equilibrium_length;
        total_energy += 0.5 * spring_force.spring_constant * elongation.powi(2);
    }

    if let Some(previous_value) = total_energy_resource.current {
        let ema_smoothing_factor = 0.2;
        let delta = timer.delta_seconds_f64();
        let alpha = (delta / ema_smoothing_factor).clamp(0.0, 1.0);
        total_energy_resource.current =
            Some(previous_value + alpha * (total_energy - previous_value));
    } else {
        total_energy_resource.current = Some(total_energy);
        total_energy_resource.initial = Some(total_energy);
    }
}
