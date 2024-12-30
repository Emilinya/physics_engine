use crate::components::{Position, Rotation, Size};
use crate::shapes::ShapeImpl;
use crate::utils::BoundingBox;

use bevy::math::DVec2;
use bevy::render::mesh::{Indices, Mesh};

#[derive(Debug, Clone, Copy)]
pub struct Square;

impl ShapeImpl for Square {
    fn get_vertices(&self) -> Vec<[f32; 2]> {
        [[-0.5, -0.5], [0.5, -0.5], [0.5, 0.5], [-0.5, 0.5]].to_vec()
    }

    fn get_mesh(&self) -> Mesh {
        self.get_incomplete_mesh()
            .with_inserted_indices(Indices::U16(vec![0, 1, 2, 0, 3, 2]))
    }

    fn get_bounding_box(&self, position: Position, size: Size, rotation: Rotation) -> BoundingBox {
        if rotation.0.abs() < 1e-6 {
            // But doctor, I am bounding box
            return BoundingBox::from_center_size(*position, DVec2::from(size));
        }

        let (sin, cos) = rotation.sin_cos();
        let bb_width = size.width * cos.abs() + size.height * sin.abs();
        let bb_height = size.width * sin.abs() + size.height * cos.abs();

        BoundingBox::from_center_size(*position, DVec2::new(bb_width, bb_height))
    }
}

impl From<Square> for Mesh {
    fn from(value: Square) -> Self {
        value.get_mesh()
    }
}
