mod circle;
mod ngon;
mod spring;
mod square;

use bevy::math::DVec2;
pub use spring::Spring as SpringShape;

use crate::components::{Position, Rotation, Size};
use crate::utils::{BoundingBox, Edge, ShapeProjection, WrappingWindows};

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

#[derive(Debug, Clone)]
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CollisionData {
    pub depth: f32,
    pub direction: Vec2,
}

#[allow(dead_code)]
impl CollisionData {
    fn new(depth: f32, direction: Vec2) -> Self {
        Self { depth, direction }
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

    fn collides_with_shape(
        &self,
        data: &ShapeData,
        other_shape: &Self,
        other_data: &ShapeData,
    ) -> Option<CollisionData> {
        self.get_shape()
            .collides_with_shape(data, other_shape, other_data)
    }
}

pub trait ShapeImpl {
    fn get_vertices(&self) -> Vec<[f32; 2]>;

    fn get_mesh(&self) -> Mesh;

    fn get_bounding_box(&self, data: &ShapeData) -> BoundingBox;

    fn collides_with_point(&self, data: &ShapeData, point: DVec2) -> bool;

    fn collides_with_shape(
        &self,
        data: &ShapeData,
        other_shape: &Shape,
        other_data: &ShapeData,
    ) -> Option<CollisionData>;

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

    fn get_shape_vertices(&self, data: &ShapeData) -> Vec<Vec2> {
        self.get_vertices()
            .iter()
            .map(|v| {
                let scaled_vec = data.size * Vec2::from_array(*v).as_dvec2();
                let rotated_vec = DVec2::from_angle(data.rotation).rotate(scaled_vec);
                (data.position + rotated_vec).as_vec2()
            })
            .collect()
    }

    /// A function to quickly see if a point is outside or inside self.
    /// Returns `true` if the point is definitely outside, and returns `false`
    /// if the point might be inside.
    fn point_definitely_outside(&self, data: &ShapeData, point: DVec2) -> bool {
        // The furthest possible distance from the center a shape can be is
        // max(width, height) / sqrt(2), which happens at the corners of a square.
        (point - data.position).length_squared() > data.size.max_element().powi(2) / 2.0
    }

    /// A function to quickly see if another shape is outside or inside self.
    /// Returns `true` if the shape is definitely outside, and returns `false`
    /// if the shape might be inside.
    fn shape_definitely_outside(
        &self,
        data: &ShapeData,
        other_shape: &Shape,
        other_data: &ShapeData,
    ) -> bool {
        !self
            .get_bounding_box(data)
            .intersects(&other_shape.get_bounding_box(other_data))
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
        check_vertices(&vertices);

        for [v1, v2] in vertices.wrapping_windows::<2>() {
            if Edge::new(v1, v2).point_outside(point - v1) {
                return false;
            }
        }

        true
    }

    fn vertex_collides_with_shape(
        &self,
        data: &ShapeData,
        other_shape: &Shape,
        other_data: &ShapeData,
    ) -> Option<CollisionData> {
        let self_vertices: Vec<_> = self.get_shape_vertices(data);
        let other_vertices: Vec<_> = other_shape.get_shape_vertices(other_data);

        #[cfg(debug_assertions)]
        check_vertices(&self_vertices);
        #[cfg(debug_assertions)]
        check_vertices(&other_vertices);

        let self_edges = self_vertices
            .wrapping_windows::<2>()
            .map(|[v1, v2]| Edge::new(v1, v2));
        let other_edges = other_vertices
            .wrapping_windows::<2>()
            .map(|[v1, v2]| Edge::new(v1, v2));
        let edge_iter = self_edges.chain(other_edges);

        let mut min_depth = f32::INFINITY;
        let mut collision_direction = Vec2::ZERO;

        for edge in edge_iter {
            let tangent = edge.tangent().normalize();
            let self_projection = ShapeProjection::project_vertices(&self_vertices, tangent);
            let other_projection = ShapeProjection::project_vertices(&other_vertices, tangent);

            let overlap = self_projection.overlap(&other_projection);
            if overlap < 0.0 {
                return None;
            }
            if overlap < min_depth {
                min_depth = overlap;
                collision_direction = tangent;
            }
        }

        // ensure direction of collision direction is correct
        let to_other = (data.position - other_data.position).as_vec2();
        if collision_direction.dot(to_other) < 0.0 {
            collision_direction = -collision_direction;
        }

        Some(CollisionData {
            depth: min_depth,
            direction: collision_direction,
        })
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

#[cfg(debug_assertions)]
/// Assert that vertices define a shape that is convex and ordered counter-clockwise
pub fn check_vertices(vertices: &[Vec2]) {
    use core::f32::consts::PI;

    let center = vertices.iter().sum::<Vec2>() / vertices.len() as f32;
    let origin_vertices: Vec<_> = vertices.iter().map(|vertex| vertex - center).collect();

    // Assert that shape is convex
    for [v1, v2, v3] in origin_vertices.wrapping_windows::<3>() {
        let to_v1 = v1 - v2;
        let to_v3 = v3 - v2;
        assert!(
            to_v1.angle_to(to_v3) < PI,
            "To use vertex based collision, shape must be convex",
        );
    }

    // Assert that vertices are ordered counter-clockwise
    for [v1, v2] in origin_vertices.wrapping_windows::<2>() {
        assert!(
            v1.angle_to(*v2) > 0.0,
            "To use vertex based collision, \
            vertices must be ordered counter-clockwise",
        );
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use crate::assert_close;

    use super::*;

    #[test]
    fn test_vertex_collides_with_point() {
        let data = ShapeData {
            position: DVec2::ZERO,
            rotation: PI / 4.0,
            size: DVec2::new(2.0, 1.0),
        };

        for (point, inside) in [
            (DVec2::new(0.84, 0.54), false),
            (DVec2::new(0.0, 0.5), true),
            (DVec2::new(-0.49, 0.25), false),
            (DVec2::new(-0.79, -0.37), false),
            (DVec2::new(-0.62, -0.46), true),
            (DVec2::new(0.18, -0.3), true),
        ] {
            assert_eq!(
                Shape::Pentagon.vertex_collides_with_point(&data, point),
                inside,
            );
        }
    }

    #[test]
    fn test_vertex_collides_with_shape() {
        let data1 = ShapeData {
            position: DVec2::ZERO,
            rotation: PI / 4.0,
            size: DVec2::new(2.0, 1.0),
        };

        let data2 = ShapeData {
            position: DVec2::ZERO,
            rotation: -PI / 6.0,
            size: DVec2::new(1.0, 2.0),
        };

        for (pos, expected_collision_data) in [
            (DVec2::new(0.77, -0.67), None),
            (
                DVec2::new(0.69, -0.59),
                Some(CollisionData::new(
                    0.038_716_326,
                    -Vec2::from_angle(-PI as f32 / 4.0),
                )),
            ),
            (DVec2::new(-1.04, 0.13), None),
            (DVec2::new(1.0, 1.5), None),
            (
                DVec2::new(0.93, 1.42),
                Some(CollisionData::new(
                    0.072_900_71,
                    -Vec2::from_angle(PI as f32 / 3.0),
                )),
            ),
            (
                DVec2::new(0.27, 0.28),
                Some(CollisionData::new(
                    0.876_761_44,
                    Vec2::new(-0.93499696, 0.35465577),
                )),
            ),
        ] {
            let other_data = ShapeData {
                position: pos,
                ..data2
            };

            let collision_data_1 =
                Shape::Pentagon.collides_with_shape(&data1, &Shape::Pentagon, &other_data);
            match (collision_data_1, expected_collision_data) {
                (Some(got), Some(expected)) => {
                    assert_close!(got.depth, expected.depth, 1e-5);
                    assert_close!(got.direction.x, expected.direction.x, 1e-5);
                    assert_close!(got.direction.y, expected.direction.y, 1e-5);
                }
                (None, None) => {}
                (other1, other2) => panic!("{other1:?} != {other2:?}"),
            }

            let collision_data_2 =
                Shape::Pentagon.collides_with_shape(&other_data, &Shape::Pentagon, &data1);
            match (collision_data_2, expected_collision_data) {
                (Some(got), Some(expected)) => {
                    assert_close!(got.depth, expected.depth, 1e-5);
                    assert_close!(got.direction.x, -expected.direction.x, 1e-5);
                    assert_close!(got.direction.y, -expected.direction.y, 1e-5);
                }
                (None, None) => {}
                (other1, other2) => panic!("{other1:?} != {other2:?}"),
            }
        }
    }
}
