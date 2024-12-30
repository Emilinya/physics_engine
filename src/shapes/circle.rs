use crate::components::{Position, Rotation, Size};
use crate::shapes::{ngon::NGon, ShapeImpl};
use crate::utils::BoundingBox;

use bevy::math::DVec2;
use bevy::render::mesh::Mesh;

#[derive(Debug, Clone, Copy)]
pub struct Circle;

impl Circle {
    // TODO: Set this to ∞
    const VERTICES: u8 = 30;
}

impl ShapeImpl for Circle {
    fn get_vertices(&self) -> Vec<[f32; 2]> {
        let ngon = NGon::<{ Self::VERTICES }>;
        ngon.get_vertices()
    }

    fn get_mesh(&self) -> Mesh {
        let ngon = NGon::<{ Self::VERTICES }>;
        ngon.get_mesh()
    }

    fn get_bounding_box(&self, position: Position, size: Size, rotation: Rotation) -> BoundingBox {
        if (size.width - size.height).abs() < 1e-6 {
            // We are a circle, who cares about rotation?
            return BoundingBox::from_center_size(*position, DVec2::splat(size.width));
        }

        let (sin, cos) = rotation.sin_cos();
        let bb_width = ((size.width * cos).powi(2) + (size.height * sin).powi(2)).sqrt();
        let bb_height = ((size.width * sin).powi(2) + (size.height * cos).powi(2)).sqrt();

        BoundingBox::from_center_size(*position, DVec2::new(bb_width, bb_height))
    }
}

impl From<Circle> for Mesh {
    fn from(value: Circle) -> Self {
        value.get_mesh()
    }
}
