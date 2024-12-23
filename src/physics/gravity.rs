use bevy::math::DVec2;
use bevy::prelude::*;

use crate::components::{PhysicsObject, Position};

// g = π²
const GRAVITY: f64 = 9.81;

pub fn apply_gravity(mut query: Query<&mut PhysicsObject>) {
    for mut physics_component in &mut query {
        physics_component.acceleration -= GRAVITY * DVec2::Y;
    }
}

pub fn gravitational_potential_energy(query: Query<(&Position, &PhysicsObject)>) -> f64 {
    query.iter().fold(0.0, |acc, (position, physics_object)| {
        acc + physics_object.mass * GRAVITY * position.0.y
    })
}
