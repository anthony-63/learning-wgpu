use game::run;

pub mod buffer;
pub mod camera;
pub mod camera_uniform;
pub mod camera_controller;
pub mod game;
pub mod math_util;
pub mod state;
pub mod texture;
pub mod instance;
pub mod instance_raw;

fn main() {
    tracing_subscriber::fmt::init();
    pollster::block_on(run());
}
