use crate::rendering::model::ModelVertex;
use crate::shapes::shape::Shape;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Square {}

impl Square {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {}
    }
}

impl Shape for Square {
    fn get_vertices(&self) -> Vec<[f32; 2]> {
        vec![[-0.5, -0.5], [-0.5, 0.5], [0.5, 0.5], [0.5, -0.5]]
    }

    fn get_model(&self) -> (Vec<ModelVertex>, Vec<u32>) {
        let indices = vec![0, 1, 2, 0, 2, 3];

        (self.to_model_vertices(), indices)
    }
}
