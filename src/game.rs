use winit::{dpi::PhysicalSize, event::*, event_loop::EventLoop, window::WindowBuilder};

pub async fn run() {
    let ev_loop = EventLoop::new();
    let win = WindowBuilder::new()
        .with_title("Learning wgpu")
        .with_inner_size(PhysicalSize::new(1280, 720))
        .build(&ev_loop)
        .unwrap();

    let mut state = crate::state::State::new(&win).await;

    ev_loop.run(move |ev, _, ctrl_flow| {
        ctrl_flow.set_wait();
        match ev {
            Event::WindowEvent { event: ev, .. } => {
                if !state.input(&ev) {
                    match ev {
                        WindowEvent::CloseRequested => ctrl_flow.set_exit(),
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    virtual_keycode: Some(virtual_code),
                                    state: ElementState::Pressed,
                                    ..
                                },
                            ..
                        } => match virtual_code {
                            VirtualKeyCode::Escape => ctrl_flow.set_exit(),
                            _ => (),
                        },
                        WindowEvent::Resized(phys_size) => state.resize(phys_size),
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            state.resize(*new_inner_size)
                        }
                        _ => (),
                    }
                }
            }
            Event::RedrawRequested(win_id) if win_id == win.id() => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => ctrl_flow.set_exit(),
                    Err(e) => eprintln!("ERROR: {:?}", e),
                }
            }
            Event::MainEventsCleared => {
                win.request_redraw();
            }
            _ => (),
        }
    });
}
