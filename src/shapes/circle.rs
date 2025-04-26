use std::f64::consts::PI;

use crate::shapes::{Shape, ShapeData, ShapeImpl, ngon::NGon, transform_point};
use crate::utils::{BoundingBox, Edge, ShapeProjection, WrappingWindows, global_newton_solver};

use bevy::math::{DVec2, Vec2};
use bevy::render::mesh::Mesh;
use nalgebra::{Matrix2, Vector2};

#[derive(Debug, Clone, Copy)]
pub struct Circle;

impl Circle {
    // TODO: Set this to ∞
    const VERTICES: u8 = 30;

    fn is_circular(data: &ShapeData) -> bool {
        // is width ≈ height?
        (data.size.x - data.size.y).abs() < 1e-6
    }

    fn circles_collide(self_data: &ShapeData, other_data: &ShapeData) -> bool {
        #![allow(non_snake_case)]

        if Self::is_circular(self_data) && Self::is_circular(other_data) {
            // circles are easy as they have a constant radius
            let self_to_other = other_data.position - self_data.position;
            let self_r = 0.5 * self_data.size.x;
            let other_r = 0.5 * self_data.size.x;
            return self_to_other.length() < (self_r + other_r);
        }

        // ellipses are surprisingly hard to deal with. To understand
        // the following computation, see <TODO: write blog>

        let S1_inv = Matrix2::new(
            1.0 / (0.5 * self_data.size.x),
            0.0,
            0.0,
            1.0 / (0.5 * self_data.size.y),
        );
        let R1_inv = {
            let (sin, cos) = self_data.rotation.sin_cos();
            Matrix2::new(cos, sin, -sin, cos)
        };
        let V = S1_inv * R1_inv;

        let S2 = Matrix2::new(0.5 * other_data.size.x, 0.0, 0.0, 0.5 * other_data.size.y);
        let R2 = {
            let (sin, cos) = other_data.rotation.sin_cos();
            Matrix2::new(cos, -sin, sin, cos)
        };

        let p = {
            let p = other_data.position - self_data.position;
            V * Vector2::new(p.x, p.y)
        };
        let M = V * R2 * S2;

        let f_fp_func = move |theta: f64| {
            let (sin, cos) = theta.sin_cos();
            let r = Vector2::new(cos, sin);
            let rp = Vector2::new(-sin, cos);

            let rT = r.transpose();
            let rpT = rp.transpose();

            let pT = p.transpose();
            let MT = M.transpose();

            let f = (p + M * r).norm_squared() - 1.0;
            let fp = rpT * MT * (p + M * r) + (pT + rT * MT) * M * rp;

            (f, fp[(0, 0)])
        };

        for theta0 in [0.0, PI] {
            let result = global_newton_solver(theta0, f_fp_func);
            if result.converged {
                return true;
            }
        }
        false
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
            Self::circles_collide(data, other_data)
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
    use core::f64::consts::PI;

    #[test]
    fn test_collides_with_circle() {
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
            // (DVec2::new(0.57, -1.01), false),
            (DVec2::new(0.4, -1.06), true),
            (DVec2::new(-1.36, -1.45), false),
            (DVec2::new(1.34, 1.47), false),
            (DVec2::new(1.29, 1.46), true),
            (DVec2::new(-0.13, 0.12), true),
            (DVec2::new(-0.81, 0.62), true),
        ] {
            let moved_data = ShapeData {
                position: pos,
                ..data2
            };
            assert_eq!(
                Shape::Circle.collides_with_shape(&data1, &Shape::Circle, &moved_data),
                collides,
            );
        }
    }

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
