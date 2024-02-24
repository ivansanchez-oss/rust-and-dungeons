use rust_and_dungeons::{input::GameInput, Player, State};
use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    window::{Theme, WindowBuilder},
};

const VELOCITY: f32 = 0.05;

#[tokio::main]
async fn main() {
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_theme(Some(Theme::Dark))
        .build(&event_loop)
        .unwrap();

    let mut state = State::new(window).await;
    let mut player = Player::new([0.0, 0.0], [0.2, 0.2]);
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
                                if input.update(key, element_state) {
                                    state.window.request_redraw();
                                }
                            }
                        },
                        WindowEvent::RedrawRequested => {
                            state.update();
                            println!("{:?}", player);

                            if input.up {
                                player.position[1] += VELOCITY;
                            }

                            if input.down {
                                player.position[1] -= VELOCITY;
                            }

                            if input.right {
                                player.position[0] += VELOCITY;
                            }

                            if input.left {
                                player.position[0] -= VELOCITY;
                            }

                            match state.render(&player) {
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
