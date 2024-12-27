use super::ngon::NGon;
use crate::shapes::shape::Shape;

use bevy::render::mesh::Mesh;

pub struct Circle;

impl Circle {
    // TODO: Set this to ∞
    const VERTICES: u8 = 30;
}

impl Shape for Circle {
    fn get_vertices(&self) -> Vec<[f32; 2]> {
        let ngon = NGon::<{ Self::VERTICES }>;
        ngon.get_vertices()
    }

    fn get_mesh(&self) -> Mesh {
        let ngon = NGon::<{ Self::VERTICES }>;
        ngon.get_mesh()
    }
}

impl From<Circle> for Mesh {
    fn from(value: Circle) -> Self {
        value.get_mesh()
    }
}
