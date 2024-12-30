use std::ops::Deref;

use bevy::math::DVec2;
use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
#[require(SpringForce, Connection)]
pub struct Spring;

#[derive(Component, Default, Clone, Copy)]
pub struct Position(pub DVec2);

impl Deref for Position {
    type Target = DVec2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Position> for DVec2 {
    fn from(value: Position) -> Self {
        value.0
    }
}

#[derive(Component, Default, Clone, Copy)]
pub struct Rotation(pub f64);

impl Deref for Rotation {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Rotation> for f64 {
    fn from(value: Rotation) -> Self {
        value.0
    }
}

#[derive(Component, Clone, Copy)]
pub struct Tangible;

#[derive(Component, Clone, Copy)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl From<Size> for DVec2 {
    fn from(value: Size) -> Self {
        Self::new(value.width, value.height)
    }
}

#[derive(Component, Clone, Copy)]
#[require(Position)]
pub struct PhysicsObject {
    pub velocity: DVec2,
    pub acceleration: DVec2,
    pub mass: f64,
}

impl PhysicsObject {
    pub fn at_rest(mass: f64) -> Self {
        Self {
            velocity: DVec2::ZERO,
            acceleration: DVec2::ZERO,
            mass,
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct SpringForce {
    pub damping: f64,
    pub spring_constant: f64,
    pub equilibrium_length: f64,
}

#[derive(Component, Clone, Copy)]
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
            damping: 0.0,
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
