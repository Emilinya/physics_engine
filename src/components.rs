use bevy::math::DVec2;
use bevy::prelude::*;

#[derive(Component)]
#[require(Position, Rotation, Size)]
pub struct Square;

#[derive(Component)]
#[require(Position, Rotation, Size, SpringForce, Connection)]
pub struct Spring;

#[derive(Component, Default)]
pub struct Position(pub DVec2);

#[derive(Component, Default)]
pub struct Rotation(pub f64);

#[derive(Component)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

#[derive(Component)]
#[require(Position)]
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

impl Default for Size {
    fn default() -> Self {
        Self {
            width: 1.0,
            height: 1.0,
        }
    }
}

impl Default for SpringForce {
    fn default() -> Self {
        Self {
            spring_constant: 1.0,
            equilibrium_length: 1.0,
        }
    }
}

impl Default for Connection {
    fn default() -> Self {
        Self {
            entity1: Entity::PLACEHOLDER,
            entity2: Entity::PLACEHOLDER,
        }
    }
}
