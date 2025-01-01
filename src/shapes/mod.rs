mod circle;
mod ngon;
mod spring;
mod square;

use bevy::math::DVec2;
pub use spring::Spring as SpringShape;

use crate::components::{Position, Rotation, Size};
use crate::utils::{BoundingBox, WrappingWindows};

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

#[derive(Debug)]
pub struct ShapeData {
    pub position: DVec2,
    pub rotation: f64,
    pub size: DVec2,
}

impl ShapeData {
    fn new(position: Position, size: Size, rotation: Rotation) -> Self {
        Self {
            position: *position,
            rotation: *rotation,
            size: DVec2::from(size),
        }
    }
}

impl From<(Position, Size, Rotation)> for ShapeData {
    fn from(value: (Position, Size, Rotation)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}

impl ShapeImpl for Shape {
    fn get_vertices(&self) -> Vec<[f32; 2]> {
        self.get_shape().get_vertices()
    }

    fn get_mesh(&self) -> Mesh {
        self.get_shape().get_mesh()
    }

    fn get_bounding_box(&self, data: &ShapeData) -> BoundingBox {
        self.get_shape().get_bounding_box(data)
    }

    fn collides_with_point(&self, data: &ShapeData, point: DVec2) -> bool {
        self.get_shape().collides_with_point(data, point)
    }
}

pub trait ShapeImpl {
    fn get_vertices(&self) -> Vec<[f32; 2]>;

    fn get_mesh(&self) -> Mesh;

    fn get_bounding_box(&self, data: &ShapeData) -> BoundingBox;

    fn collides_with_point(&self, data: &ShapeData, point: DVec2) -> bool;

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

    /// A function to quickly see if a point is outside or inside a shape.
    /// Returns `true` if the point is definitely outside, and returns `false`
    /// if the point might be inside.
    fn point_definitely_outside(&self, data: &ShapeData, point: DVec2) -> bool {
        // The furthest possible distance from the center a shape can be is
        // max(width, height) / sqrt(2), which happens at the corners of a square.
        (point - data.position).length_squared() > data.size.max_element().powi(2) / 2.0
    }

    /// Get bounding box by iterating over all vertices.
    fn vertex_get_bounding_box(&self, data: &ShapeData) -> BoundingBox {
        let size_vec = data.size.as_vec2();
        let pos_vec = data.position.as_vec2();

        let vertices: Vec<_> = self
            .get_vertices()
            .iter()
            .map(|v| {
                let vertex = Vec2::from_array(*v);
                Vec2::from_angle(data.rotation as f32).rotate(vertex * size_vec)
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

        BoundingBox::from_corners(top_right.as_dvec2(), bottom_left.as_dvec2())
    }

    /// See if point is inside shape by iterating over all vertices.
    /// Note: This assumes shape is convex and vertices are ordered counter-clockwise
    fn vertex_collides_with_point(&self, data: &ShapeData, point: DVec2) -> bool {
        let point = transform_point(data, point).as_vec2();
        let vertices: Vec<_> = self
            .get_vertices()
            .iter()
            .map(|v| Vec2::from_array(*v))
            .collect();

        #[cfg(debug_assertions)]
        {
            use core::f32::consts::PI;

            // Assert that shape is convex
            for [v1, v2, v3] in vertices.wrapping_windows::<3>() {
                let to_v1 = v1 - v2;
                let to_v3 = v3 - v2;
                assert!(
                    to_v1.angle_to(to_v3) < PI,
                    "To use `vertex_collides_with_point`, \
                    shape must be convex"
                );
            }

            // Assert that vertices are ordered counter-clockwise
            for [v1, v2] in vertices.wrapping_windows::<2>() {
                assert!(
                    v1.angle_to(*v2) > 0.0,
                    "To use `vertex_collides_with_point`, \
                    vertices must be ordered counter-clockwise"
                );
            }
        }

        for [v1, v2] in vertices.wrapping_windows::<2>() {
            let to_point = point - v1;
            let edge_vec = v2 - v1;
            let tangent = Vec2::new(edge_vec.y, -edge_vec.x);

            if tangent.dot(to_point) > 0.0 {
                return false;
            }
        }

        true
    }
}

/// Transform a point relative to some object with a position, size,
/// and rotation such that the position is effectively (0, 0), the size
/// is effectively (1, 1), and the rotation is effectively 0.
pub fn transform_point(data: &ShapeData, point: DVec2) -> DVec2 {
    // translate point so position is effectively (0, 0)
    let point = point - data.position;
    // rotate vector so rotation is effectively 0
    let point = DVec2::from_angle(-data.rotation).rotate(point);
    // scale vector so size is effectively (1, 1)
    point / data.size
}
