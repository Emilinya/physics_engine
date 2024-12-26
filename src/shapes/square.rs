use crate::shapes::shape::Shape;

use bevy::render::mesh::{Indices, Mesh};

pub struct Square;

impl Shape for Square {
    fn get_vertices(&self) -> Vec<[f32; 2]> {
        [[-0.5, -0.5], [0.5, -0.5], [0.5, 0.5], [-0.5, 0.5]].to_vec()
    }

    fn get_mesh(&self) -> Mesh {
        self.get_incomplete_mesh()
            .with_inserted_indices(Indices::U16(vec![0, 1, 2, 0, 3, 2]))
    }
}

impl From<Square> for Mesh {
    fn from(value: Square) -> Self {
        value.get_mesh()
    }
}
