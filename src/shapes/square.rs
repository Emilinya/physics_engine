use crate::shapes::{Shape, ShapeData, ShapeImpl, transform_point};
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

    fn get_bounding_box(&self, data: &ShapeData) -> BoundingBox {
        if data.rotation.abs() < 1e-6 {
            // But doctor, I am bounding box
            return BoundingBox::from_center_size(data.position, data.size);
        }

        let (sin, cos) = data.rotation.sin_cos();
        let bb_width = data.size.x * cos.abs() + data.size.y * sin.abs();
        let bb_height = data.size.x * sin.abs() + data.size.y * cos.abs();

        BoundingBox::from_center_size(data.position, DVec2::new(bb_width, bb_height))
    }

    fn collides_with_point(&self, data: &ShapeData, point: DVec2) -> bool {
        if self.point_definitely_outside(data, point) {
            return false;
        }

        transform_point(data, point)
            .abs()
            .cmplt(DVec2::splat(0.5))
            .all()
    }

    fn collides_with_shape(
        &self,
        data: &ShapeData,
        other_shape: &Shape,
        other_data: &ShapeData,
    ) -> bool {
        if self.shape_definitely_outside(data, other_shape, other_data) {
            return false;
        }

        true
    }
}

impl From<Square> for Mesh {
    fn from(value: Square) -> Self {
        value.get_mesh()
    }
}
