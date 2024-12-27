use bevy::math::DVec2;
use bevy::prelude::*;

use crate::components::{Connection, PhysicsObject, Position, Rotation, Size, Spring, SpringForce};

fn get_spring_connection_positions(
    connection: &Connection,
    position_query: &Query<&Position, Without<Spring>>,
) -> Option<(DVec2, DVec2)> {
    let Ok(pos1) = position_query.get(connection.entity1).map(|p| p.0) else {
        return None;
    };
    let Ok(pos2) = position_query.get(connection.entity2).map(|p| p.0) else {
        return None;
    };

    Some((pos1, pos2))
}

pub fn apply_spring_force(
    spring_query: Query<(&SpringForce, &Connection)>,
    position_query: Query<&Position, Without<Spring>>,
    mut physics_query: Query<&mut PhysicsObject>,
) {
    for (spring_force, connection) in &spring_query {
        let Some((pos1, pos2)) = get_spring_connection_positions(connection, &position_query)
        else {
            continue;
        };

        let between = pos2 - pos1;
        let length = between.length();
        let direction = between / length;

        let displacement = length - spring_force.equilibrium_length;
        let force = spring_force.spring_constant * direction * displacement;

        for entity in [connection.entity1, connection.entity2] {
            let Ok(mut physics) = physics_query.get_mut(entity) else {
                continue;
            };
            let mass = physics.mass;
            let damping = spring_force.damping * physics.velocity;

            if entity == connection.entity1 {
                physics.acceleration += (force - damping) / mass;
            } else {
                physics.acceleration += (-force - damping) / mass;
            }
        }
    }
}

pub fn update_spring(
    mut commands: Commands,
    mut spring_query: Query<
        (Entity, &Connection, &mut Position, &mut Size, &mut Rotation),
        With<Spring>,
    >,
    position_query: Query<&Position, Without<Spring>>,
) {
    for (entity, connection, mut position, mut size, mut rotation) in &mut spring_query {
        let Some((pos1, pos2)) = get_spring_connection_positions(connection, &position_query)
        else {
            debug!("Despawning spring with invalid connections");
            commands.entity(entity).despawn();
            continue;
        };

        let midpoint = (pos1 + pos2) / 2.0;
        let between = pos1 - pos2;
        let length = between.length();

        position.0 = midpoint;
        rotation.0 = DVec2::angle_to(DVec2::X, between);
        size.width = length;
    }
}

pub fn spring_potential_energy(
    spring_query: Query<(&SpringForce, &Connection)>,
    position_query: Query<&Position, Without<Spring>>,
) -> f64 {
    let mut total_energy = 0.0;
    for (spring_force, connection) in &spring_query {
        let Some((pos1, pos2)) = get_spring_connection_positions(connection, &position_query)
        else {
            continue;
        };

        let elongation = (pos2 - pos1).length() - spring_force.equilibrium_length;
        total_energy += 0.5 * spring_force.spring_constant * elongation.powi(2);
    }
    total_energy
}
