use crate::components::{Position, Rotation, Size};
use crate::shapes::ShapeImpl;

use bevy::math::{DVec2, Rect};
use bevy::render::mesh::{Indices, Mesh};

pub struct Square;

impl ShapeImpl for Square {
    fn get_vertices(&self) -> Vec<[f32; 2]> {
        [[-0.5, -0.5], [0.5, -0.5], [0.5, 0.5], [-0.5, 0.5]].to_vec()
    }

    fn get_mesh(&self) -> Mesh {
        self.get_incomplete_mesh()
            .with_inserted_indices(Indices::U16(vec![0, 1, 2, 0, 3, 2]))
    }

    fn get_bounding_box(&self, position: &Position, size: &Size, rotation: &Rotation) -> Rect {
        if rotation.0.abs() < 1e-6 {
            // But doctor, I am bounding box
            return Rect::from_center_size(position.0.as_vec2(), DVec2::from(size).as_vec2());
        }

        let (sin, cos) = rotation.0.sin_cos();

        let bb_width = size.width * cos.abs() + size.height * sin.abs();
        let bb_height = size.width * sin.abs() + size.height * cos.abs();

        let top_right = position.0 + DVec2::new(bb_width / 2.0, bb_height / 2.0);
        let bottom_left = position.0 + DVec2::new(-bb_width / 2.0, -bb_height / 2.0);
        Rect::from_corners(top_right.as_vec2(), bottom_left.as_vec2())
    }
}

impl From<Square> for Mesh {
    fn from(value: Square) -> Self {
        value.get_mesh()
    }
}
