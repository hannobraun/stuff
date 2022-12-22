use winit::{event::Event, event_loop::EventLoop, window::Window};

pub async fn run() -> anyhow::Result<()> {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop)?;

    event_loop.run(move |event, _, _| match event {
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {
            dbg!(event);
        }
    })
}
