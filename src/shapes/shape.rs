use core::cell::Ref;
use core::cmp::{max_by, min_by};

use cgmath::Angle;

use crate::shapes::{circle::Circle, ngon::NGon, slope::Slope, spring::Spring, square::Square};
use crate::{instance::Instance, rendering::model::ModelVertex};

pub trait Shape {
    fn to_model_vertices(&self) -> Vec<ModelVertex> {
        self.get_vertices()
            .iter()
            .map(|pos| ModelVertex {
                position: *pos,
                tex_coords: [pos[0] + 0.5, pos[1] + 0.5],
                normal: [0.0, 0.0],
            })
            .collect()
    }
    fn get_vertices(&self) -> Vec<[f32; 2]>;
    fn get_model(&self) -> (Vec<ModelVertex>, Vec<u32>);
    fn vertex_bounding_box(
        &self,
        entity: &Ref<Instance>,
    ) -> (cgmath::Vector2<f32>, cgmath::Vector2<f32>) {
        // this function might be expensive

        let transformation_matrix = entity.get_model_matrix(false);
        let entity_vertices: Vec<cgmath::Vector2<f32>> = self
            .get_vertices()
            .iter()
            .map(|v| {
                let vec3 = transformation_matrix * cgmath::Vector3::new(v[0], v[1], 1.0);
                cgmath::Vector2::new(vec3.x, vec3.y)
            })
            .collect();

        let mut top_right = entity_vertices[0];
        let mut bottom_left = entity_vertices[0];
        for v in &entity_vertices[1..] {
            top_right.x = max_by(top_right.x, v.x, f32::total_cmp);
            top_right.y = max_by(top_right.y, v.y, f32::total_cmp);
            bottom_left.x = min_by(bottom_left.x, v.x, f32::total_cmp);
            bottom_left.y = min_by(bottom_left.y, v.y, f32::total_cmp);
        }

        (top_right, bottom_left)
    }
    fn get_bounding_box(
        &self,
        entity: &Ref<Instance>,
    ) -> (cgmath::Vector2<f32>, cgmath::Vector2<f32>) {
        // bounding box of a rectangle
        let (sin, cos) = entity.rotation.sin_cos();

        let bb_width = entity.width * cos.abs() + entity.height * sin.abs();
        let bb_height = entity.width * sin.abs() + entity.height * cos.abs();

        let top_right = entity.position + cgmath::Vector2::new(bb_width / 2.0, bb_height / 2.0);
        let bottom_left = entity.position + cgmath::Vector2::new(-bb_width / 2.0, -bb_height / 2.0);
        (top_right, bottom_left)
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum ShapeEnum {
    #[allow(dead_code)]
    Square(Square),
    #[allow(dead_code)]
    Slope(Slope),
    #[allow(dead_code)]
    NGon(NGon),
    #[allow(dead_code)]
    Circle(Circle),
    #[allow(dead_code)]
    Spring(Spring),
}

impl ShapeEnum {
    pub fn get_model(&self) -> (Vec<ModelVertex>, Vec<u32>) {
        match self {
            ShapeEnum::NGon(ngon) => ngon.get_model(),
            ShapeEnum::Slope(slope) => slope.get_model(),
            ShapeEnum::Square(square) => square.get_model(),
            ShapeEnum::Circle(circle) => circle.get_model(),
            ShapeEnum::Spring(spring) => spring.get_model(),
        }
    }

    #[allow(dead_code)]
    pub fn get_bounding_box(
        &self,
        entity: &Ref<Instance>,
    ) -> (cgmath::Vector2<f32>, cgmath::Vector2<f32>) {
        match self {
            ShapeEnum::NGon(ngon) => ngon.get_bounding_box(entity),
            ShapeEnum::Slope(slope) => slope.get_bounding_box(entity),
            ShapeEnum::Square(square) => square.get_bounding_box(entity),
            ShapeEnum::Circle(circle) => circle.get_bounding_box(entity),
            ShapeEnum::Spring(spring) => spring.get_bounding_box(entity),
        }
    }
}
