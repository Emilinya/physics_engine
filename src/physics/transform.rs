use bevy::prelude::*;

use crate::components::{Position, Rotation, Size};

pub fn update_transform(
    camera_query: Query<&Camera>,
    mut transform_query: Query<(&mut Transform, &Position, &Size, &Rotation)>,
) {
    let camera = camera_query.get_single().unwrap();
    let viewport_size = camera
        .logical_viewport_size()
        .expect("Can't get viewport size?!");
    let scale = viewport_size.x.min(viewport_size.y);

    for (mut transform, position, size, rotation) in &mut transform_query {
        let z = transform.translation.z;
        *transform = Transform {
            translation: Vec3::new(position.0.x as f32 * scale, position.0.y as f32 * scale, z),
            rotation: Quat::from_rotation_z(rotation.0 as f32),
            scale: Vec3::new(size.width as f32 * scale, size.height as f32 * scale, 1.0),
        };
    }
}
