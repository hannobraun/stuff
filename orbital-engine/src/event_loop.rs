use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::{host::Host, renderer::Renderer};

pub async fn run() -> anyhow::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_maximized(true)
        .build(&event_loop)?;
    let renderer = Renderer::new(&window).await?;
    let host = Host::new().await?;

    let color = [0., 0., 0., 1.];

    let mut handler = EventLoopHandler {
        window,
        renderer,
        host,
        color,
    };

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(size) => handler
                .renderer
                .update_surface_size(size.width, size.height),
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }
            WindowEvent::KeyboardInput { input, .. } => {
                match input.virtual_keycode {
                    Some(VirtualKeyCode::Escape) => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                }
            }
            _ => {}
        },
        Event::MainEventsCleared => {
            handler.color = handler.host.color();
            handler.window.request_redraw();
        }
        Event::RedrawRequested(_) => {
            handler.renderer.draw(handler.color).unwrap();
        }
        _ => {}
    })
}

struct EventLoopHandler {
    window: Window,
    renderer: Renderer,
    host: Host,
    color: [f64; 4],
}
