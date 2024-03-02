mod game;
mod render;

use crate::render::controller::RenderController;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget};
use winit::window::Window;

pub struct GameEngine {
    render: RenderController,
    game: game::GameController,
}

impl GameEngine {
    pub fn new(render: RenderController, game: game::GameController) -> Self {
        Self { render, game }
    }

    pub fn window(&self) -> &Window {
        &self.render.window()
    }
}

pub fn event_handler<T>(
    engine: &mut GameEngine,
    event: Event<T>,
    target: &EventLoopWindowTarget<T>,
) {
    if let Event::WindowEvent { window_id, event } = event {
        if engine.window().id() == window_id {
            window_event_handler(engine, event, target);
        }
    }
}

pub fn window_event_handler<T>(
    engine: &mut GameEngine,
    event: WindowEvent,
    target: &EventLoopWindowTarget<T>,
) {
    match event {
        WindowEvent::Resized(size) => engine.render.resize(size),
        WindowEvent::CloseRequested => target.exit(),
        WindowEvent::RedrawRequested => handle_redraw(engine, target),
        _ => {}
    }
}

pub fn handle_redraw<T>(engine: &mut GameEngine, target: &EventLoopWindowTarget<T>) {
    if let Err(err) = engine.render.render(&engine.game) {
        match err {
            wgpu::SurfaceError::Lost => engine.render.resize(engine.window().inner_size()),
            // The system is out of memory, we should probably quit
            wgpu::SurfaceError::OutOfMemory => target.exit(),
            // All other errors (Outdated, Timeout) should be resolved by the next frame
            e => eprintln!("{:?}", e),
        }
    }
}

pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    let window = Window::new(&event_loop)?;

    let render = RenderController::build(window).await?;
    let game = game::GameController::new();
    let mut engine = GameEngine::new(render, game);

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run(move |event, target| event_handler(&mut engine, event, target))?;

    Ok(())
}
