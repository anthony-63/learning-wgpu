use winit::event::{WindowEvent, KeyboardInput, ElementState, VirtualKeyCode};

use crate::camera::Camera;

#[derive(Debug)]
pub struct CameraController {
    speed: f32,
    going_forward: bool,
    going_backward: bool,
    going_left: bool,
    going_right: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            going_forward: false,
            going_backward: false,
            going_left: false,
            going_right: false,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput { input: KeyboardInput { state,virtual_keycode: Some(keycode), ..}, .. } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::W => { self.going_forward = is_pressed; true}
                    VirtualKeyCode::S => { self.going_backward = is_pressed; true}
                    VirtualKeyCode::A => { self.going_left = is_pressed; true}
                    VirtualKeyCode::D => { self.going_right = is_pressed; true}
                    _ => false,           
                }
            },

            _ => false,
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        use cgmath::InnerSpace;
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        if self.going_forward && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        }
        if self.going_backward {
            camera.eye -= forward_norm * self.speed;
        }
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();

        let right = forward_norm.cross(camera.up);

        if self.going_right {
            camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.going_left {
            camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }
    }
}