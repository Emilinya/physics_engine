use std::collections::HashMap;
use std::f64::consts::PI;

use crate::shapes::{CollisionData, Shape, ShapeData, ShapeImpl, ngon::NGon, transform_point};
use crate::utils::{
    BoundingBox, DEdge, Edge, ShapeProjection, ToVec, ToVector, WrappingWindows,
    global_newton_solver, solve_quadratic,
};

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

    fn circles_collide(self_data: &ShapeData, other_data: &ShapeData) -> Option<CollisionData> {
        #![allow(non_snake_case)]

        if Self::is_circular(self_data) && Self::is_circular(other_data) {
            // circles are easy as they have a constant radius
            let self_to_other = other_data.position - self_data.position;
            let self_r = 0.5 * self_data.size.x;
            let other_r = 0.5 * self_data.size.x;
            let overlap = (self_r + other_r) - self_to_other.length();
            if overlap > 0.0 {
                return Some(CollisionData {
                    depth: overlap as f32,
                    direction: self_to_other.normalize().as_vec2(),
                });
            } else {
                return None;
            };
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

        let p = V * (other_data.position - self_data.position).to_vector();
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

        const THRESHOLD: f64 = 1e6;
        let mut intersection_angles = HashMap::new();
        for theta0 in [0.0, PI / 2.0, PI, 3.0 * PI / 2.0] {
            let result = global_newton_solver(theta0, f_fp_func);
            if !result.converged {
                continue;
            }

            // ensure angle is in [0, 2𝜋]. I can't use % because programmers are dumb
            let excess_pies = (result.value / (2.0 * PI)).floor();
            let angle = result.value - excess_pies * 2.0 * PI;
            intersection_angles.insert((angle * THRESHOLD).floor() as i64, angle);
        }
        if intersection_angles.len() < 2 {
            return None;
        }

        let intersection_points = intersection_angles
            .values()
            .map(|angle| {
                let r = DVec2::from_angle(*angle).to_vector();
                other_data.position + (R2 * S2 * r).to_vec()
            })
            .collect::<Vec<_>>();

        let (collision_point, mut collision_direction) = match &intersection_points[..] {
            [point1, point2] => (
                (point1 + point2) / 2.0,
                DEdge::new(point1, point2).tangent().normalize(),
            ),
            other => {
                log::warn!(
                    "Got {} collision points? Intersection angles: {:?}",
                    other.len(),
                    intersection_angles
                );
                return None;
            }
        };

        // ensure direction of collision_direction is correct
        let to_other = self_data.position - other_data.position;
        if collision_direction.dot(to_other) < 0.0 {
            collision_direction = -collision_direction;
        }

        // to find the collision depth, we need to do stuff. Draw a figure if you want to understand.
        let get_orthogonal_intercept = |S_inv: Matrix2<f64>, R_inv: Matrix2<f64>, P: DVec2| {
            let M = S_inv * R_inv;
            let alpha = M * collision_direction.to_vector();
            let beta = M * (collision_point - P).to_vector();

            let A = alpha.norm_squared();
            let B = 2.0 * alpha.dot(&beta);
            let C = beta.norm_squared() - 1.0;

            match &solve_quadratic(A, B, C)[..] {
                [root] => Ok(collision_point + collision_direction * root),
                [root1, root2] => {
                    let p1 = collision_point + collision_direction * root1;
                    let p2 = collision_point + collision_direction * root2;
                    // choose root that is closest to collision point
                    if (p1 - collision_point).length_squared()
                        < (p2 - collision_point).length_squared()
                    {
                        Ok(p1)
                    } else {
                        Ok(p2)
                    }
                }
                other => Err(format!(
                    "got unexpected number of solutions: {}",
                    other.len()
                )),
            }
        };

        let point1 = get_orthogonal_intercept(S1_inv, R1_inv, self_data.position);
        let point2 = get_orthogonal_intercept(
            S2.try_inverse().expect("scale matrix should be invertible"),
            R2.transpose(),
            other_data.position,
        );

        let (Ok(point1), Ok(point2)) = (&point1, &point2) else {
            log::warn!(
                "Failed to get orthogonal intercept: {:?}/{:?}",
                point1,
                point2
            );
            return None;
        };

        let depth = (point1 - point2).length();

        Some(CollisionData::new(
            depth as f32,
            collision_direction.as_vec2(),
        ))
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
    ) -> Option<CollisionData> {
        if self.shape_definitely_outside(data, other_shape, other_data) {
            return None;
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

            let mut min_depth = f32::INFINITY;
            let mut collision_direction = Vec2::ZERO;

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
}

impl From<Circle> for Mesh {
    fn from(value: Circle) -> Self {
        value.get_mesh()
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_close;

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

        for (pos, expected_collision_data) in [
            (DVec2::new(0.57, -1.01), None),
            (
                DVec2::new(0.4, -1.06),
                Some(CollisionData::new(
                    0.013_327_299,
                    Vec2::from_angle(5.459_973_4),
                )),
            ),
            (DVec2::new(-1.36, -1.45), None),
            (DVec2::new(1.34, 1.47), None),
            (
                DVec2::new(1.29, 1.46),
                Some(CollisionData::new(
                    0.024_825_816,
                    Vec2::from_angle(0.652_652_47),
                )),
            ),
            (
                DVec2::new(-0.13, 0.12),
                Some(CollisionData::new(
                    0.199_056_25,
                    Vec2::new(-0.6857814, 0.7278076),
                )),
            ),
            (
                DVec2::new(-0.81, 0.62),
                Some(CollisionData::new(
                    0.005_188_186_5,
                    Vec2::from_angle(2.487_424),
                )),
            ),
        ] {
            let moved_data = ShapeData {
                position: pos,
                ..data2
            };

            let collision_data_1 =
                Shape::Circle.collides_with_shape(&moved_data, &Shape::Circle, &data1);
            match (collision_data_1, expected_collision_data) {
                (Some(got), Some(expected)) => {
                    assert_close!(got.depth, expected.depth, 1e-5);
                    assert_close!(got.direction.x, expected.direction.x, 1e-5);
                    assert_close!(got.direction.y, expected.direction.y, 1e-5);
                }
                (None, None) => {}
                (other1, other2) => panic!("{:?} != {:?}", other1, other2),
            }

            let collision_data_2 =
                Shape::Circle.collides_with_shape(&data1, &Shape::Circle, &moved_data);
            match (collision_data_2, expected_collision_data) {
                (Some(got), Some(expected)) => {
                    assert_close!(got.depth, expected.depth, 1e-5);
                    assert_close!(got.direction.x, -expected.direction.x, 1e-5);
                    assert_close!(got.direction.y, -expected.direction.y, 1e-5);
                }
                (None, None) => {}
                (other1, other2) => panic!("{:?} != {:?}", other1, other2),
            }
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

        for (pos, expected_collision_data) in [
            (DVec2::new(0.81, -0.61), None),
            (
                DVec2::new(0.62, -0.71),
                Some(CollisionData::new(
                    0.011_998_534,
                    -Vec2::from_angle(135.0 * PI as f32 / 180.0),
                )),
            ),
            (DVec2::new(-1.0, -1.57), None),
            (DVec2::new(-1.02, 0.2), None),
            (
                DVec2::new(-0.96, 0.17),
                Some(CollisionData::new(
                    0.033_792_555,
                    Vec2::from_angle(2.704_643),
                )),
            ),
            (
                DVec2::new(-0.26, 0.28),
                Some(CollisionData::new(
                    0.621_475_76,
                    Vec2::from_angle(2.704_643),
                )),
            ),
            (DVec2::new(1.17, 1.59), None),
        ] {
            let moved_data = ShapeData {
                position: pos,
                ..data2
            };

            let collision_data_1 =
                Shape::Circle.collides_with_shape(&moved_data, &Shape::Pentagon, &data1);
            match (collision_data_1, expected_collision_data) {
                (Some(got), Some(expected)) => {
                    assert_close!(got.depth, expected.depth, 1e-5);
                    assert_close!(got.direction.x, expected.direction.x, 1e-5);
                    assert_close!(got.direction.y, expected.direction.y, 1e-5);
                }
                (None, None) => {}
                (other1, other2) => panic!("{:?} != {:?}", other1, other2),
            }

            let collision_data_2 =
                Shape::Pentagon.collides_with_shape(&data1, &Shape::Circle, &moved_data);
            match (collision_data_2, expected_collision_data) {
                (Some(got), Some(expected)) => {
                    assert_close!(got.depth, expected.depth, 1e-5);
                    assert_close!(got.direction.x, -expected.direction.x, 1e-5);
                    assert_close!(got.direction.y, -expected.direction.y, 1e-5);
                }
                (None, None) => {}
                (other1, other2) => panic!("{:?} != {:?}", other1, other2),
            }
        }
    }
}
