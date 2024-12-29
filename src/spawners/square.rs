use crate::components::*;

use bevy::math::DVec2;

pub fn physics_square_bundle(
    mass: f64,
    width: f64,
    height: f64,
    position: DVec2,
) -> (Position, Size, PhysicsObject, Tangible) {
    (
        Position(position),
        Size { width, height },
        PhysicsObject::at_rest(mass),
        Tangible,
    )
}
