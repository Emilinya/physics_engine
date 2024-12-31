use bevy::prelude::*;

use crate::components::{Position, Rotation, Size};
use crate::WindowSize;

pub fn update_transform(
    window: Res<WindowSize>,
    mut transform_query: Query<(&mut Transform, &Position, &Size, &Rotation)>,
) {
    for (mut transform, position, size, rotation) in &mut transform_query {
        let z = transform.translation.z;
        *transform = Transform {
            translation: Vec3::new(
                position.0.x as f32 * window.scale,
                position.0.y as f32 * window.scale,
                z,
            ),
            rotation: Quat::from_rotation_z(rotation.0 as f32),
            scale: Vec3::new(
                size.width as f32 * window.scale,
                size.height as f32 * window.scale,
                1.0,
            ),
        };
    }
}
