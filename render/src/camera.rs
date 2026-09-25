use glam::{
    Mat4, Vec3,
    camera::rh::{proj::directx as webgpu, view},
};

pub struct Camera {
    eye: Vec3,
    center: Vec3,
    up: Vec3,
    aspect_ratio: f32,
    vertical_fov: f32,
    near: f32,
    far: f32,
}

impl Camera {
    pub fn new(
        eye: Vec3,
        center: Vec3,
        up: Vec3,
        aspect_ratio: f32,
        vertical_fov: f32,
        near: f32,
        far: f32,
    ) -> Self {
        Camera {
            eye,
            center,
            up,
            aspect_ratio,
            vertical_fov,
            near,
            far,
        }
    }

    pub fn eye(&mut self, eye: Vec3) {
        self.eye = eye
    }

    pub fn center(&mut self, center: Vec3) {
        self.center = center
    }

    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let view = view::look_at_mat4(self.eye, self.center, self.up);
        let proj = webgpu::perspective(self.vertical_fov, self.aspect_ratio, self.near, self.far);
        proj * view
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().to_cols_array_2d()
    }
}
