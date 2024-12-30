mod circle;
mod ngon;
mod spring;
mod square;

pub use spring::Spring as SpringShape;

use crate::components::{Position, Rotation, Size};

use bevy::prelude::*;
use bevy::render::{
    mesh::Mesh, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology,
};
use {circle::Circle, ngon::NGon, spring::Spring, square::Square};

#[allow(dead_code)]
#[derive(Component, Debug, Clone, Copy)]
#[require(Position, Rotation, Size)]
pub enum Shape {
    Spring(Spring),
    Circle,
    Square,
    Triangle,
    Pentagon,
    Hexagon,
    Heptagon,
    Octagon,
}

impl Shape {
    fn get_shape(&self) -> &dyn ShapeImpl {
        match self {
            Self::Spring(spring) => spring,
            Self::Circle => &Circle,
            Self::Square => &Square,
            Self::Triangle => &NGon::<3>,
            Self::Pentagon => &NGon::<5>,
            Self::Hexagon => &NGon::<6>,
            Self::Heptagon => &NGon::<7>,
            Self::Octagon => &NGon::<8>,
        }
    }
}

impl ShapeImpl for Shape {
    fn get_vertices(&self) -> Vec<[f32; 2]> {
        self.get_shape().get_vertices()
    }

    fn get_mesh(&self) -> Mesh {
        self.get_shape().get_mesh()
    }

    fn get_bounding_box(&self, position: &Position, size: &Size, rotation: &Rotation) -> Rect {
        self.get_shape().get_bounding_box(position, size, rotation)
    }
}

pub trait ShapeImpl {
    fn get_vertices(&self) -> Vec<[f32; 2]>;

    fn get_mesh(&self) -> Mesh;

    fn get_bounding_box(&self, position: &Position, size: &Size, rotation: &Rotation) -> Rect;

    /// Create `Mesh` with position, uv, and normals, but not indices.
    fn get_incomplete_mesh(&self) -> Mesh {
        let vertices = self.get_vertices();

        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vertices
                .iter()
                .map(|pos| [pos[0], pos[1], 0.0])
                .collect::<Vec<[f32; 3]>>(),
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            // vertices are in [-0.5, 0.5], transform them to be [0, 1]
            vertices
                .iter()
                .map(|pos| [pos[0] + 0.5, pos[1] + 0.5])
                .collect::<Vec<[f32; 2]>>(),
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            [[0.0, 0.0, 1.0]].repeat(vertices.len()),
        )
    }

    /// Get bounding box by iterating over all vertices, which can be quite slow.
    fn vertex_bounding_box(&self, position: &Position, size: &Size, rotation: &Rotation) -> Rect {
        let size_vec = Vec2::new(size.width as f32, size.height as f32);
        let pos_vec = position.0.as_vec2();

        let vertices: Vec<_> = self
            .get_vertices()
            .iter()
            .map(|v| {
                let vertex = Vec2::from_array(*v);
                Vec2::from_angle(rotation.0 as f32).rotate(vertex * size_vec)
            })
            .collect();

        let mut top_right = vertices[0];
        let mut bottom_left = vertices[0];
        for vertex in &vertices[1..] {
            top_right.x = top_right.x.max(vertex.x);
            top_right.y = top_right.y.max(vertex.y);
            bottom_left.x = bottom_left.x.min(vertex.x);
            bottom_left.y = bottom_left.y.min(vertex.y);
        }

        top_right += pos_vec;
        bottom_left += pos_vec;

        Rect::from_corners(top_right, bottom_left)
    }
}
