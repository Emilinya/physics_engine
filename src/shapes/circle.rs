use crate::components::{Position, Rotation, Size};
use crate::shapes::{ngon::NGon, Shape, ShapeImpl};

use bevy::math::{Rect, Vec2};
use bevy::render::mesh::Mesh;

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

    fn get_bounding_box(&self, position: &Position, size: &Size, rotation: &Rotation) -> Rect {
        if (size.width - size.height).abs() < 1e-6 {
            // We are a circle, who cares about rotation?
            Rect::from_center_size(position.0.as_vec2(), Vec2::splat(size.width as f32))
        } else {
            // TODO: I think we can do better than this
            Shape::Square.get_bounding_box(position, size, rotation)
        }
    }
}

impl From<Circle> for Mesh {
    fn from(value: Circle) -> Self {
        value.get_mesh()
    }
}
