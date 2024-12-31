use crate::shapes::{ngon::NGon, transform_point, ShapeData, ShapeImpl};
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
        NGon::<{ Self::VERTICES }>.get_vertices()
    }

    fn get_mesh(&self) -> Mesh {
        NGon::<{ Self::VERTICES }>.get_mesh()
    }

    fn get_bounding_box(&self, data: ShapeData) -> BoundingBox {
        if (data.size.x - data.size.y).abs() < 1e-6 {
            // We are a circle, who cares about rotation?
            return BoundingBox::from_center_size(data.position, data.size);
        }

        let (sin, cos) = data.rotation.sin_cos();
        let bb_width = ((data.size.x * cos).powi(2) + (data.size.y * sin).powi(2)).sqrt();
        let bb_height = ((data.size.x * sin).powi(2) + (data.size.y * cos).powi(2)).sqrt();

        BoundingBox::from_center_size(data.position, DVec2::new(bb_width, bb_height))
    }

    fn collides_with_point(&self, data: ShapeData, point: DVec2) -> bool {
        // When size is (1, 1), diameter is 1, so radius is 0.5
        let r = 0.5;

        if (data.size.x - data.size.y).abs() < 1e-6 {
            // circle-point collision is easy
            return (data.position - point).length_squared() < (r * data.size.x).powi(2);
        }

        transform_point(data, point).length_squared() < r.powi(2)
    }
}

impl From<Circle> for Mesh {
    fn from(value: Circle) -> Self {
        value.get_mesh()
    }
}
