use core::cell::Ref;

use cgmath::Angle;

use crate::instance::Instance;
use crate::rendering::model::ModelVertex;
use crate::shapes::shape::Shape;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct NGon {
    point_count: usize,
}

impl NGon {
    pub fn new(point_count: usize) -> Self {
        Self { point_count }
    }
}

impl Shape for NGon {
    fn get_vertices(&self) -> Vec<[f32; 2]> {
        let mut points = Vec::with_capacity(self.point_count);

        for i in 0..self.point_count {
            let (sin, cos) = cgmath::Deg(-((i * 360) as f32 / self.point_count as f32)).sin_cos();
            points.push([0.5 * cos, 0.5 * sin]);
        }

        points
    }

    fn get_model(&self) -> (Vec<ModelVertex>, Vec<u32>) {
        let num_indices = 3 * (self.point_count - 2);
        let mut indices = Vec::with_capacity(num_indices);

        for i in 0..(num_indices / 3) {
            indices.push(0);
            indices.push((i + 1) as u32);
            indices.push((i + 2) as u32);
        }

        (self.to_model_vertices(), indices)
    }

    fn get_bounding_box(
        &self,
        entity: &Ref<Instance>,
    ) -> (cgmath::Vector2<f32>, cgmath::Vector2<f32>) {
        self.vertex_bounding_box(entity)
    }
}
