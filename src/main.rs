use rust_and_dungeons::{input::GameInput, State};
use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    window::{Theme, WindowBuilder},
};

#[tokio::main]
async fn main() {
    let event_loop = EventLoop::new().unwrap();

    let w = WindowBuilder::new()
        .with_theme(Some(Theme::Dark))
        .build(&event_loop)
        .unwrap();

    let mut state = State::new(w).await;
    let mut input = GameInput::default();

    event_loop
        .run(|e, elwt| {
            match e {
                Event::WindowEvent { window_id, event } => {
                    if window_id != state.window.id() {
                        return;
                    }

                    match event {
                        WindowEvent::Resized(new_size) => state.resize(new_size),
                        WindowEvent::CloseRequested => elwt.exit(),
                        WindowEvent::KeyboardInput { event, .. } => match event {
                            KeyEvent {
                                state: element_state,
                                logical_key: key,
                                ..
                            } => {
                                input.update(key, element_state);
                            }
                        },
                        WindowEvent::RedrawRequested => {
                            state.update();
                            match state.render() {
                                Ok(_) => {}
                                // Reconfigure the surface if lost
                                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                                // The system is out of memory, we should probably quit
                                Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                                // All other errors (Outdated, Timeout) should be resolved by the next frame
                                Err(e) => eprintln!("{:?}", e),
                            }
                        }
                        _ => {}
                    }
                }

                _ => {}
            }
        })
        .unwrap();
}
