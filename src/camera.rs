#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct Camera {
    view: cgmath::Matrix4<f32>,
    proj: cgmath::Matrix4<f32>,
    pub mat: cgmath::Matrix4<f32>,
}

impl Camera {
    fn create_view_matrix(height: f32) -> cgmath::Matrix4<f32> {
        cgmath::Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, -1.0, 0.0, 0.0,
            0.0, 0.0, -1.0, 0.0,
            0.0, 0.0, -height, 1.0,
        )
    }

    fn create_proj_matrix(aspect: f32) -> cgmath::Matrix4<f32> {
        let fov = 120.0;
        let znear = 0.1;
        let zfar = 100.0;

        cgmath::perspective(cgmath::Deg(fov), aspect, znear, zfar)
    }

    pub fn new(height: f32, aspect: f32) -> Self {
        let view = Self::create_view_matrix(height);
        let proj = Self::create_proj_matrix(aspect);
        Self {view, proj, mat: proj * view}
    }

    #[allow(dead_code)]
    pub fn update_height(&mut self, height: f32) {
        self.view = Self::create_view_matrix(height);
        self.mat = self.proj * self.view;
    }

    pub fn update_aspect(&mut self, aspect: f32) {
        self.proj = Self::create_proj_matrix(aspect);
        self.mat = self.proj * self.view;
    }

    pub fn get_world_size(&self) -> (f32, f32) {
        let x_vec = cgmath::Vector4::new(1.0, 0.0, 0.0, 1.0);
        let y_vec = cgmath::Vector4::new(0.0, 1.0, 0.0, 1.0);
        let view_proj = OPENGL_TO_WGPU_MATRIX * self.mat;

        (1.0 / (view_proj * x_vec).x, -1.0 / (view_proj * y_vec).y)
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn from_camera(camera: &Camera) -> Self {
        Self {
            view_proj: (OPENGL_TO_WGPU_MATRIX * camera.mat).into(),
        }
    }
}
