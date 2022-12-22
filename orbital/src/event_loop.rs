use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::renderer::Renderer;

pub async fn run() -> anyhow::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_maximized(true)
        .build(&event_loop)?;
    let mut renderer = Renderer::new(&window).await?;

    let mut color = [0., 0., 0., 1.];

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(size) => {
                renderer.surface_config.width = size.width;
                renderer.surface_config.height = size.height;

                renderer
                    .surface
                    .configure(&renderer.device, &renderer.surface_config);
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
            color = [0., 0., 0., 1.];
            window.request_redraw();
        }
        Event::RedrawRequested(_) => {
            let [r, g, b, a] = color;

            let mut encoder = renderer.device.create_command_encoder(
                &wgpu::CommandEncoderDescriptor { label: None },
            );
            let surface_texture =
                renderer.surface.get_current_texture().unwrap();
            let view = surface_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r, g, b, a }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            renderer.queue.submit([encoder.finish()]);
            surface_texture.present();
        }
        _ => {}
    })
}
