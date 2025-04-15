use std::f64::consts::PI;

use crate::shapes::{Shape, ShapeData, ShapeImpl, ngon::NGon, transform_point};
use crate::utils::{BoundingBox, Edge, ShapeProjection, WrappingWindows};

use bevy::math::{DVec2, Vec2};
use bevy::render::mesh::Mesh;

#[derive(Debug, Clone, Copy)]
pub struct Circle;

impl Circle {
    // TODO: Set this to ∞
    const VERTICES: u8 = 30;

    fn is_circular(data: &ShapeData) -> bool {
        // is width ≈ height?
        (data.size.x - data.size.y).abs() < 1e-6
    }

    fn get_radius(data: &ShapeData, along: DVec2) -> f64 {
        if Self::is_circular(data) {
            // circles are easy :)
            0.5 * data.size.x
        } else if data.rotation.abs() < 1e-6 {
            // ellipses are still fine if they are not rotated :)
            let (a, b) = (0.5 * data.size.x, 0.5 * data.size.y);
            let (sin, cos) = along.to_angle().sin_cos();
            (a * cos).hypot(b * sin)
        } else {
            // rotated ellipses are scary :(
            let (a, b) = (0.5 * data.size.x, 0.5 * data.size.y);
            let cos = if along.x.abs() < 1e-6 {
                // when along = (0, y), along.to_angle().cos() == 0,
                // so along.to_angle().tan() "=" ∞. Luckily, we can calculate
                // an analytical value for cos in this case
                -(2.0 * data.rotation).cos()
            } else {
                // add pi so angle is in the range [0, 2π]
                let phi = along.to_angle() + PI;
                // TODO: this seems inefficient :(
                let offset = (a / b * phi.tan()).atan();
                (2.0 * (data.rotation - offset)).cos()
            };

            // The proof of this formula is left as an exercise to the reader
            let (a2, b2) = (a.powi(2), b.powi(2));
            ((cos * (a2 - b2) + a2 + b2) / 2.0).sqrt()
        }
    }
}

impl ShapeImpl for Circle {
    fn get_vertices(&self) -> Vec<[f32; 2]> {
        NGon::<{ Self::VERTICES }>.get_vertices()
    }

    fn get_mesh(&self) -> Mesh {
        NGon::<{ Self::VERTICES }>.get_mesh()
    }

    fn get_bounding_box(&self, data: &ShapeData) -> BoundingBox {
        if Self::is_circular(data) {
            // We are a circle, who cares about rotation?
            return BoundingBox::from_center_size(data.position, data.size);
        }

        let (sin, cos) = data.rotation.sin_cos();
        let bb_width = (data.size.x * cos).hypot(data.size.y * sin);
        let bb_height = (data.size.x * sin).hypot(data.size.y * cos);

        BoundingBox::from_center_size(data.position, DVec2::new(bb_width, bb_height))
    }

    fn collides_with_point(&self, data: &ShapeData, point: DVec2) -> bool {
        if self.point_definitely_outside(data, point) {
            return false;
        }

        // When size is (1, 1), diameter is 1, so radius is 0.5
        let r = 0.5;

        if Self::is_circular(data) {
            // circle-point collision is easy
            return (data.position - point).length_squared() < (r * data.size.x).powi(2);
        }

        transform_point(data, point).length_squared() < r.powi(2)
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

        if matches!(other_shape, Shape::Circle) {
            let self_to_other = other_data.position - data.position;
            let self_r = Self::get_radius(data, self_to_other);
            let other_r = Self::get_radius(other_data, -self_to_other);
            (data.position - other_data.position).length() < (self_r + other_r)
        } else {
            let other_vertices: Vec<_> = other_shape.get_shape_vertices(other_data);

            #[cfg(debug_assertions)]
            super::check_vertices(&other_vertices);

            let self_tangent = {
                let mut min_distance = f32::INFINITY;
                let mut to_closest_point = Vec2::ZERO;

                for vertex in &other_vertices {
                    let to_point = vertex - data.position.as_vec2();
                    let length = to_point.length_squared();
                    if length < min_distance {
                        min_distance = length;
                        to_closest_point = to_point;
                    }
                }

                to_closest_point.normalize()
            };

            let other_tangents = other_vertices
                .wrapping_windows::<2>()
                .map(|[v1, v2]| Edge::new(v1, v2).tangent().normalize());
            let tangent_iter = other_tangents.chain([self_tangent]);

            for tangent in tangent_iter {
                let pos_dot = data.position.as_vec2().dot(tangent);
                let self_projection = if Self::is_circular(data) {
                    let height = data.size.x as f32;
                    ShapeProjection::from_min_max(pos_dot - 0.5 * height, pos_dot + 0.5 * height)
                } else {
                    // So, why angle to (0, -1)? Well... draw it yourself and find out
                    let tangent_angle = tangent.as_dvec2().angle_to(DVec2::NEG_Y);

                    let (sin, cos) = (data.rotation + tangent_angle).sin_cos();
                    let height = (data.size.x * sin).hypot(data.size.y * cos) as f32;
                    ShapeProjection::from_min_max(pos_dot - 0.5 * height, pos_dot + 0.5 * height)
                };
                let other_projection = ShapeProjection::project_vertices(&other_vertices, tangent);

                if !self_projection.overlaps_with(&other_projection) {
                    return false;
                }
            }

            true
        }
    }
}

impl From<Circle> for Mesh {
    fn from(value: Circle) -> Self {
        value.get_mesh()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collides_with_shape() {
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

        for (pos, collides) in [
            (DVec2::new(0.81, -0.61), false),
            (DVec2::new(0.62, -0.71), true),
            (DVec2::new(-1.0, -1.57), false),
            (DVec2::new(-1.02, 0.2), false),
            (DVec2::new(-0.96, 0.17), true),
            (DVec2::new(-0.26, 0.28), true),
            (DVec2::new(1.17, 1.59), false),
        ] {
            let moved_data = ShapeData {
                position: pos,
                ..data2
            };
            assert_eq!(
                Shape::Circle.collides_with_shape(&moved_data, &Shape::Pentagon, &data1),
                collides,
            );
        }
    }
}
