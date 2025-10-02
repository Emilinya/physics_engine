use std::collections::HashMap;

use bevy::prelude::*;

use crate::components::{PhysicsObject, Position, Rotation, Size, Tangible};
use crate::shapes::{CollisionData, Shape, ShapeImpl};

const COLLISION_STRENGTH: f32 = 500.0;

pub fn apply_collision_force(
    mut physics_query: Query<&mut PhysicsObject>,
    shape_query: Query<(Entity, &Shape, &Position, &Size, &Rotation), With<Tangible>>,
) {
    let mut collision_data_map: HashMap<Entity, Vec<CollisionData>> = HashMap::new();

    for (entity1, shape1, position1, size1, rotation1) in &shape_query {
        let data1 = (*position1, *size1, *rotation1).into();
        for (entity2, shape2, position2, size2, rotation2) in &shape_query {
            if entity1 == entity2 {
                continue;
            }
            let data2 = (*position2, *size2, *rotation2).into();

            if let Some(collision_data) = shape1.collides_with_shape(&data1, shape2, &data2) {
                let entry = collision_data_map.entry(entity1).or_default();
                entry.push(collision_data);
            }
        }
    }

    for (entity, collision_data_lists) in collision_data_map {
        let Ok(mut physics_object) = physics_query.get_mut(entity) else {
            continue;
        };

        let mass = physics_object.mass;
        for collision_data in collision_data_lists {
            physics_object.acceleration +=
                (COLLISION_STRENGTH * collision_data.depth * collision_data.direction).as_dvec2()
                    / mass;
        }
    }
}

pub fn collision_potential_energy(
    colliding_query: Query<(Entity, &Shape, &Position, &Size, &Rotation), With<Tangible>>,
) -> f64 {
    let mut collision_data_map: HashMap<Entity, Vec<CollisionData>> = HashMap::new();

    for (entity1, shape1, position1, size1, rotation1) in &colliding_query {
        let data1 = (*position1, *size1, *rotation1).into();
        for (entity2, shape2, position2, size2, rotation2) in &colliding_query {
            if entity1 == entity2 {
                continue;
            }
            let data2 = (*position2, *size2, *rotation2).into();

            if let Some(collision_data) = shape1.collides_with_shape(&data1, shape2, &data2) {
                let entry = collision_data_map.entry(entity1).or_default();
                entry.push(collision_data);
            }
        }
    }

    let mut total_collision_energy = 0.0;
    for collision_data_lists in collision_data_map.values() {
        for collision_data in collision_data_lists {
            let energy = 0.5 * COLLISION_STRENGTH * collision_data.depth.powi(2);
            total_collision_energy += energy as f64;
        }
    }

    // divide by two as we count each collision twice
    total_collision_energy / 2.0
}
