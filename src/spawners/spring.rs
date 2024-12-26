use crate::components::*;

use bevy::prelude::*;

pub fn spring_bundle(
    width: f64,
    entity1: Entity,
    entity2: Entity,
    spring_constant: f64,
    equilibrium_length: f64,
) -> (Spring, Size, SpringForce, Connection) {
    (
        Spring,
        Size {
            width: 0.0,
            height: width,
        },
        SpringForce {
            spring_constant,
            equilibrium_length,
        },
        Connection { entity1, entity2 },
    )
}
