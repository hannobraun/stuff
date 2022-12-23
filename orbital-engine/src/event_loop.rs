use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{host::Host, renderer::Renderer};

pub async fn run() -> anyhow::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_maximized(true)
        .build(&event_loop)?;
    let mut renderer = Renderer::new(&window).await?;
    let mut host = Host::new().await?;

    let mut color = [0., 0., 0., 1.];

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(size) => {
                renderer.update_surface_size(size.width, size.height)
            }
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
            color = host.color();
            window.request_redraw();
        }
        Event::RedrawRequested(_) => {
            renderer.draw(color).unwrap();
        }
        _ => {}
    })
}
