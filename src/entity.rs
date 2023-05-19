use std::rc::Rc;

use crate::model;

pub struct Entity {
    pub model: Rc<model::Model>,
    pub position: cgmath::Vector2<f32>,
    pub rotation: cgmath::Rad<f32>,
    pub width: f32,
    pub height: f32,
}

impl Entity {
    pub fn get_model_matrix(&self, reverse_y: bool) -> cgmath::Matrix3<f32> {
        // positive y is up, not down!
        let corrected_position = {
            if reverse_y {
                let y_correction_matrix = cgmath::Matrix2::new(
                    1.0, 0.0,
                    0.0, -1.0,
                );
                y_correction_matrix * self.position
            } else {
                self.position
            }
        };

        // why is this neccesary? because cos(-θ) = cos(θ), sin(-θ) = -sin(θ) => this also flips y
        let corrected_rotation = {
            if reverse_y {
                -self.rotation
            } else {
                self.rotation
            }
        };

        let translation_matrix = cgmath::Matrix3::from_translation(corrected_position);
        let rotation_matrix = cgmath::Matrix3::from_angle_z(corrected_rotation);
        let scale_matrix = cgmath::Matrix3::from_nonuniform_scale(self.width, self.height);

        translation_matrix * rotation_matrix * scale_matrix
    }

    pub fn to_raw(&self) -> EntityModel {
        EntityModel {
            model: self.get_model_matrix(true).into(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct EntityModel {
    #[allow(dead_code)]
    model: [[f32; 3]; 3],
}

impl EntityModel {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<EntityModel>() as wgpu::BufferAddress,
            // We need to switch from using a step mode of Vertex to Instance
            // This means that our shaders will only change to use the next
            // Instance when the shader starts processing a new Instance
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    // While our vertex shader only uses locations 0, and 1 now, in later tutorials we'll
                    // be using 2, 3, and 4, for Vertex. We'll start at slot 5 not conflict with them later
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // A mat4 takes up 4 vertex slots as it is technically 4 vec4s. We need to define a slot
                // for each vec4. We don't have to do this in code though.
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}
