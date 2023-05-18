use core::cell::Ref;
use core::cmp::{max_by, min_by};

use cgmath::Angle;

use crate::{model::ModelVertex, entity::Entity};

#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Square,
    Slope,
    NGon(usize),
    Circle,
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
        }
    }

    pub fn get_bounding_box(&self, entity: Ref<Entity>) -> (cgmath::Vector2<f32>, cgmath::Vector2<f32>) {
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

                let transformation_matrix = entity.get_model_matrix();
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
                        (entity.width, entity.height)
                    } else {
                        let (sin, cos ) = entity.rotation.sin_cos();
        
                        let bb_width = entity.width * cos.abs() + entity.height * sin.abs();
                        let bb_height = entity.width * sin.abs() + entity.height * cos.abs();
                        (bb_width, bb_height)
                    }
                };

                let top_right = entity.position + cgmath::Vector2::new(bb_width / 2.0, bb_height / 2.0);
                let bottom_left = entity.position + cgmath::Vector2::new(-bb_width / 2.0, -bb_height / 2.0);
                (top_right, bottom_left)
            }
        }
    }
}
