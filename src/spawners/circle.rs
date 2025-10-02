use crate::components::{PhysicsObject, Position, Size, Tangible};

use bevy::math::DVec2;

pub const fn physics_circle_bundle(
    mass: f64,
    radius: f64,
    position: DVec2,
) -> (Position, Size, PhysicsObject, Tangible) {
    (
        Position(position),
        Size {
            width: radius,
            height: radius,
        },
        PhysicsObject::at_rest(mass),
        Tangible,
    )
}
