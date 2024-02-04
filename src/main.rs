use log::debug;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, EventLoopWindowTarget},
    window::{Theme, Window, WindowBuilder},
};

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();

    let mut w = WindowBuilder::new()
        .with_theme(Some(Theme::Dark))
        .build(&event_loop)
        .unwrap();

    event_loop.run(|e, elwt| handler(&mut w, e, elwt)).unwrap();
}

fn handler(window: &mut Window, event: Event<()>, elwt: &EventLoopWindowTarget<()>) {
    debug!("{:?}", event);
    match event {
        Event::WindowEvent { window_id, event } => {
            if window_id != window.id() {
                return;
            }

            match event {
                WindowEvent::CloseRequested => elwt.exit(),
                _ => {}
            }
        }
        _ => {}
    }
}
