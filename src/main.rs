use game::run;

pub mod game;
pub mod state;
fn main() {
    tracing_subscriber::fmt::init();
    pollster::block_on(run());
}
