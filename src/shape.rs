use core::cell::Ref;
use core::cmp::{max_by, min_by};
use std::f32::consts::PI;

use cgmath::Angle;

use crate::{model::ModelVertex, entity::Entity};

const D: f32 = 0.02;
const CC: f32 = 20.0;

fn spring_top_curve(t: f32) -> (f32, f32) {
    let (sin, cos) =  cgmath::Rad(CC*2.0*PI*t).sin_cos();
    let f = 0.5*(1.0-D)*sin;
    let fp = 0.5*(1.0-D)*CC*2.0*PI*cos;
    let nfp = (1.0 + fp.powi(2)).sqrt();

    (t - 0.5*D*fp/nfp, f + 0.5*D/nfp)
}

fn spring_bottom_curve(t: f32) -> (f32, f32) {
    let (sin, cos) =  cgmath::Rad(CC*2.0*PI*t).sin_cos();
    let f = 0.5*(1.0-D)*sin;
    let fp = 0.5*(1.0-D)*CC*2.0*PI*cos;
    let nfp = (1.0 + fp.powi(2)).sqrt();

    (t + 0.5*D*fp/nfp, f - 0.5*D/nfp)
}

fn interpolate(x: f32, points: &Vec<(f32, f32)>) -> f32 {
    let mut p = 0.0;
    for i in 0..points.len() {
        let mut l = 1.0;
        for j in 0..points.len() {
            if j == i {
                continue;
            }
            l *= (x - points[j].0)/(points[i].0 - points[j].0);
        }
        p += l * points[i].1;
    }
    return p;
}

fn curve2function(x: f32, f: impl Fn(f32) -> (f32, f32)) -> f32 {
    let (fist_px, fist_py) = f(x);
    if (fist_px - x).abs() < 1e-8 {
        return fist_py;
    }

    let delta = 0.01 / (D.sqrt() * CC.powi(2));
    let sign = (x - fist_px).signum();

    let mut nx = x + delta * sign;
    while sign == (x - f(nx).0).signum() {
        nx += delta * sign;
    }

    let points = vec![
        f(nx - delta * sign),
        f(nx - 0.25*delta * sign),
        f(nx - 0.5*delta * sign),
        f(nx - 0.75*delta * sign),
        f(nx),
    ];

    return interpolate(x, &points);
}

#[derive(Debug, Clone, Copy)]
pub enum Shape {
    #[allow(dead_code)]
    Square,
    #[allow(dead_code)]
    Slope,
    #[allow(dead_code)]
    NGon(usize),
    #[allow(dead_code)]
    Circle,
    #[allow(dead_code)]
    Spring,
}

impl Shape {
    const CIRCLE_POINT_COUNT: usize = 50;

    fn to_model_vertices(&self) -> Vec<ModelVertex> {
        self.get_vertices().iter().map(|pos| ModelVertex {
            position: *pos,
            tex_coords: [pos[0] + 0.5, pos[1] + 0.5],
            normal: [0.0, 0.0],
        }).collect()
    }

    pub fn get_vertices(&self) -> Vec<[f32; 2]> {
        match self {
            Shape::Square => vec![[-0.5, -0.5], [-0.5, 0.5], [0.5, 0.5], [0.5, -0.5]],
            Shape::Slope => vec![[-0.5, -0.5], [-0.5, 0.5], [0.5, -0.5]],
            Shape::NGon(num_points) => {
                let mut points = Vec::with_capacity(*num_points);

                for i in 0..*num_points {
                    let (sin, cos) = cgmath::Deg(-((i * 360) as f32 / *num_points as f32)).sin_cos();
                    points.push([0.5*cos, 0.5*sin]);
                }

                points
            }
            Shape::Circle => Shape::NGon(Self::CIRCLE_POINT_COUNT).get_vertices(),
            Shape::Spring => {
                const N: u32 = 20*CC as u32;
                let num_points = N*2 + 1;
                let dx = 1.0 / (N - 1) as f32;

                let mut points = Vec::with_capacity(num_points as usize);
                points.push([-0.5, curve2function(-0.5, spring_top_curve)]);
                points.push([-0.5, curve2function(-0.5, spring_bottom_curve)]);
                for i in 1..N {
                    let x = -0.5 + (i as f32)*dx;
                    points.push([x, curve2function(x, spring_top_curve)]);
                    points.push([x - 0.5*dx, curve2function(x - 0.5*dx, spring_bottom_curve)]);
                }
                points.push([0.5, curve2function(0.5, spring_bottom_curve)]);

                points
            }
        }
    }

    pub fn get_model(&self) -> (Vec<ModelVertex>, Vec<u32>) {
        match self {
            Shape::Square => {
                let indices = vec![
                    0, 1, 2,
                    0, 2, 3,
                ];

                (self.to_model_vertices(), indices)
            },
            Shape::Slope => {
                let indices = vec![
                    0, 1, 2,
                ];

                (self.to_model_vertices(), indices)
            },
            Shape::NGon(num_points) => {
                let num_indices = 3 * (num_points - 2);
                let mut indices = Vec::with_capacity(num_indices);

                for i in 0..(num_indices / 3) {
                    indices.push(0);
                    indices.push((i + 1) as u32);
                    indices.push((i + 2) as u32);
                }

                (self.to_model_vertices(), indices)
            },
            Shape::Circle => Shape::NGon(Self::CIRCLE_POINT_COUNT).get_model(),
            Shape::Spring => {
                const N: u32 = 20*CC as u32;
                let num_indices = 3*(2*N-1);
                let mut indices = Vec::with_capacity(num_indices as usize);

                for i in 0..N {
                    if i != 0 {
                        indices.push(2*i);
                        indices.push(2*i + 1);
                        indices.push(2*(i - 1));
                    }
                    if i != N-1 {
                        indices.push(2*i + 3);
                        indices.push(2*i + 1);
                        indices.push(2*i);
                    }
                }
                indices.push(2*N);
                indices.push(2*N-1);
                indices.push(2*N-2);

                (self.to_model_vertices(), indices)
            }
        }
    }

    pub fn get_bounding_box(&self, entity: &Ref<Entity>) -> (cgmath::Vector2<f32>, cgmath::Vector2<f32>) {
        match self {
            Shape::Square => {
                let (sin, cos ) = entity.rotation.sin_cos();

                let bb_width = entity.width * cos.abs() + entity.height * sin.abs();
                let bb_height = entity.width * sin.abs() + entity.height * cos.abs();

                let top_right = entity.position + cgmath::Vector2::new(bb_width / 2.0, bb_height / 2.0);
                let bottom_left = entity.position + cgmath::Vector2::new(-bb_width / 2.0, -bb_height / 2.0);
                (top_right, bottom_left)
            },
            Shape::Slope => Shape::Square.get_bounding_box(entity),
            Shape::NGon(_) => {
                let comp = |x: &f32, y: &f32| x.total_cmp(y);

                let transformation_matrix = entity.get_model_matrix(false);
                let entity_vertices: Vec<cgmath::Vector2<f32>> = self.get_vertices().iter().map(|v| {
                    let vec3 = transformation_matrix * cgmath::Vector3::new(v[0], v[1], 1.0);
                    cgmath::Vector2::new(vec3.x, vec3.y)
                }).collect();

                let mut top_right = entity_vertices[0];
                let mut bottom_left = entity_vertices[0];
                for v in &entity_vertices[1..] {
                    top_right.x = max_by(top_right.x, v.x, comp);
                    top_right.y = max_by(top_right.y, v.y, comp);
                    bottom_left.x = min_by(bottom_left.x, v.x, comp);
                    bottom_left.y = min_by(bottom_left.y, v.y, comp);
                }

                (top_right, bottom_left)
            },
            Shape::Circle => {
                let (bb_width, bb_height) = {
                    if (entity.width == entity.height) | (entity.rotation == cgmath::Rad(0.0)) {
                        // shape is a circle, bounding box is very simple
                        (entity.width, entity.height)
                    } else {
                        // shape is a rotated elipse, bounding box is complicated
                        let (sin, cos ) = entity.rotation.sin_cos();
                        let bb_width = ((entity.width * cos).powi(2) + (entity.height * sin).powi(2)).sqrt();
                        let bb_height = ((entity.width * sin).powi(2) + (entity.height * cos).powi(2)).sqrt();
        
                        (bb_width, bb_height)
                    }
                };

                let top_right = entity.position + cgmath::Vector2::new(bb_width / 2.0, bb_height / 2.0);
                let bottom_left = entity.position + cgmath::Vector2::new(-bb_width / 2.0, -bb_height / 2.0);
                (top_right, bottom_left)
            },
            Self::Spring => Shape::Square.get_bounding_box(entity),
        }
    }
}
