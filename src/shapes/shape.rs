use bevy::render::{
    mesh::Mesh, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology,
};

pub trait Shape {
    fn get_mesh(&self) -> Mesh;

    fn get_vertices(&self) -> Vec<[f32; 2]>;

    fn get_uv_map(&self) -> Option<Vec<[f32; 2]>> {
        None
    }

    /// Create `Mesh` with position, uv, and normals, but not indices.
    fn get_incomplete_mesh(&self) -> Mesh {
        let vertices = self.get_vertices();

        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vertices
                .iter()
                .map(|pos| [pos[0], pos[1], 0.0])
                .collect::<Vec<[f32; 3]>>(),
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            self.get_uv_map()
                .or_else(|| {
                    Some(
                        // vertices are in [-0.5, 0.5], transform them to be [0, 1]
                        vertices
                            .iter()
                            .map(|pos| [pos[0] + 0.5, pos[1] + 0.5])
                            .collect(),
                    )
                })
                .unwrap(),
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            [[0.0, 0.0, 1.0]].repeat(vertices.len()),
        )
    }

    // fn vertex_bounding_box(
    //     &self,
    //     entity: &Ref<Instance>,
    // ) -> (cgmath::Vector2<f32>, cgmath::Vector2<f32>) {
    //     // this function might be expensive

    //     let transformation_matrix = entity.get_model_matrix(false);
    //     let entity_vertices: Vec<cgmath::Vector2<f32>> = self
    //         .get_vertices()
    //         .iter()
    //         .map(|v| {
    //             let vec3 = transformation_matrix * cgmath::Vector3::new(v[0], v[1], 1.0);
    //             cgmath::Vector2::new(vec3.x, vec3.y)
    //         })
    //         .collect();

    //     let mut top_right = entity_vertices[0];
    //     let mut bottom_left = entity_vertices[0];
    //     for v in &entity_vertices[1..] {
    //         top_right.x = max_by(top_right.x, v.x, f32::total_cmp);
    //         top_right.y = max_by(top_right.y, v.y, f32::total_cmp);
    //         bottom_left.x = min_by(bottom_left.x, v.x, f32::total_cmp);
    //         bottom_left.y = min_by(bottom_left.y, v.y, f32::total_cmp);
    //     }

    //     (top_right, bottom_left)
    // }

    // fn get_bounding_box(
    //     &self,
    //     entity: &Ref<Instance>,
    // ) -> (cgmath::Vector2<f32>, cgmath::Vector2<f32>) {
    //     // bounding box of a rectangle
    //     let (sin, cos) = entity.rotation.sin_cos();

    //     let bb_width = entity.width * cos.abs() + entity.height * sin.abs();
    //     let bb_height = entity.width * sin.abs() + entity.height * cos.abs();

    //     let top_right = entity.position + cgmath::Vector2::new(bb_width / 2.0, bb_height / 2.0);
    //     let bottom_left = entity.position + cgmath::Vector2::new(-bb_width / 2.0, -bb_height / 2.0);
    //     (top_right, bottom_left)
    // }
}
