use bevy::math::DVec2;
use bevy::prelude::*;

#[derive(Component)]
pub struct Square;

#[derive(Component)]
pub struct Spring;

#[derive(Component)]
pub struct Position(pub DVec2);

#[derive(Component)]
pub struct Rotation(pub f64);

#[derive(Component)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

#[derive(Component)]
pub struct PhysicsObject {
    pub velocity: DVec2,
    pub acceleration: DVec2,
    pub mass: f64,
}

impl PhysicsObject {
    pub fn at_rest(mass: f64) -> Self {
        PhysicsObject {
            velocity: DVec2::ZERO,
            acceleration: DVec2::ZERO,
            mass,
        }
    }
}

#[derive(Component)]
pub struct SpringForce {
    pub spring_constant: f64,
    pub equilibrium_length: f64,
}

#[derive(Component)]
pub struct Connection {
    pub entity1: Entity,
    pub entity2: Entity,
}
