use bevy::prelude as bvy;
use bevy::math::DVec2;

#[derive(bvy::Component)]
pub struct Square;


#[derive(bvy::Component)]
pub struct Spring;

#[derive(bvy::Component)]
pub struct Position(pub DVec2);

#[derive(bvy::Component)]
pub struct Rotation(pub f64);

#[derive(bvy::Component)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

#[derive(bvy::Component)]
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

#[derive(bvy::Component)]
pub struct SpringForce {
    pub spring_constant: f64,
    pub equilibrium_length: f64,
}

#[derive(bvy::Component)]
pub struct Connection {
    pub entity1: bvy::Entity,
    pub entity2: bvy::Entity,
}
