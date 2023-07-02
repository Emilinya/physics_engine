use core::cell::Ref;

use cgmath::Angle;

use crate::rendering::{model::ModelVertex, instance::Instance};
use crate::shapes::{ngon::NGon, shape::Shape};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Circle {}

impl Circle {
    const POINT_COUNT: usize = 50;

    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {}
    }
}

impl Shape for Circle {
    fn get_vertices(&self) -> Vec<[f32; 2]> {
        NGon::new(Self::POINT_COUNT).get_vertices()
    }

    fn get_model(&self) -> (Vec<ModelVertex>, Vec<u32>) {
        NGon::new(Self::POINT_COUNT).get_model()
    }

    fn get_bounding_box(
        &self,
        entity: &Ref<Instance>,
    ) -> (cgmath::Vector2<f32>, cgmath::Vector2<f32>) {
        let (bb_width, bb_height) = {
            if (entity.width == entity.height) | (entity.rotation == cgmath::Rad(0.0)) {
                // shape is a circle, bounding box is very simple
                (entity.width, entity.height)
            } else {
                // shape is a rotated elipse, bounding box is complicated
                let (sin, cos) = entity.rotation.sin_cos();
                let bb_width = (
                    (entity.width * cos).powi(2) + (entity.height * sin).powi(2)
                ).sqrt();
                let bb_height = (
                    (entity.width * sin).powi(2) + (entity.height * cos).powi(2)
                ).sqrt();

                (bb_width, bb_height)
            }
        };

        let top_right = entity.position + cgmath::Vector2::new(bb_width / 2.0, bb_height / 2.0);
        let bottom_left = entity.position + cgmath::Vector2::new(-bb_width / 2.0, -bb_height / 2.0);
        (top_right, bottom_left)
    }
}
