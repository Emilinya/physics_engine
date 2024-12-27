use core::f32::consts::PI;

use crate::shapes::shape::Shape;

use bevy::render::mesh::{Indices, Mesh};

pub struct NGon<const N: u8>;

impl<const N: u8> Shape for NGon<N> {
    fn get_vertices(&self) -> Vec<[f32; 2]> {
        let mut vertices = Vec::with_capacity(N as usize);
        let delta_angle = (2.0 * PI) / N as f32;
        for i in 0..N {
            // Subtract angle by pi/2 so it is rotated correctly
            let angle = delta_angle * (i as f32) + PI / 2.0;
            let (sin, cos) = angle.sin_cos();
            vertices.push([0.5 * cos, 0.5 * sin]);
        }
        vertices
    }

    fn get_mesh(&self) -> Mesh {
        // It would be nice if this was an compile-time error
        if N < 3 {
            panic!("Can't construct an NGon with less than 3 vertices");
        }

        let mut indices = Vec::with_capacity(3 * ((N - 2) as usize));
        for i in 0..(N - 2) {
            indices.push(0);
            indices.push(i as u16 + 1);
            indices.push(i as u16 + 2);
        }
        self.get_incomplete_mesh()
            .with_inserted_indices(Indices::U16(indices))
    }
}

impl<const N: u8> From<NGon<N>> for Mesh {
    fn from(value: NGon<N>) -> Self {
        value.get_mesh()
    }
}
