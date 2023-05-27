use core::cell::Ref;

use cgmath::Zero;

use crate::entity::Entity;
use crate::rendering::model::ModelVertex;
use crate::shapes::{shape::Shape, square::Square};

#[derive(Debug, Clone, Copy)]
pub struct Slope {}

impl Slope {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {}
    }
}

impl Shape for Slope {
    fn get_vertices(&self) -> Vec<[f32; 2]> {
        vec![[-0.5, -0.5], [-0.5, 0.5], [0.5, -0.5]]
    }

    fn get_model(&self) -> (Vec<ModelVertex>, Vec<u32>) {
        let indices = vec![0, 1, 2];

        (self.to_model_vertices(), indices)
    }

    fn get_bounding_box(
        &self,
        entity: &Ref<Entity>,
    ) -> (cgmath::Vector2<f32>, cgmath::Vector2<f32>) {
        if entity.rotation.is_zero() {
            Square::new().get_bounding_box(entity)
        } else {
            self.vertex_bounding_box(entity)
        }
    }
}
