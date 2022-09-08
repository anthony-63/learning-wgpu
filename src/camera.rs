pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect_ratio: f32,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn build_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        use crate::math_util::OPENGL_TO_WGPU_MATRIX;
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(
            cgmath::Deg(self.fov),
            self.aspect_ratio,
            self.near,
            self.far,
        );
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}
