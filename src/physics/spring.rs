use bevy::math::DVec2;
use bevy::prelude::*;

use crate::components::{Connection, PhysicsObject, Position, Rotation, Size, Spring, SpringForce};

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

pub fn apply_spring_force(
    spring_query: Query<(&SpringForce, &Connection)>,
    position_query: Query<&Position, Without<Spring>>,
    mut physics_query: Query<&mut PhysicsObject>,
) {
    for (spring_force, connection) in &spring_query {
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

pub fn update_spring(
    mut spring_query: Query<(&Connection, &mut Position, &mut Size, &mut Rotation), With<Spring>>,
    position_query: Query<&Position, Without<Spring>>,
) {
    for (connection, mut position, mut size, mut rotation) in &mut spring_query {
        let (pos1, pos2) = get_spring_connection_positions(connection, &position_query);

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
        let (pos1, pos2) = get_spring_connection_positions(connection, &position_query);

        let elongation = (pos2 - pos1).length() - spring_force.equilibrium_length;
        total_energy += 0.5 * spring_force.spring_constant * elongation.powi(2);
    }
    total_energy
}
