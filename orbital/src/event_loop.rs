use anyhow::anyhow;
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

    let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await
        .ok_or_else(|| anyhow!("Could not request adapter"))?;
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::downlevel_webgl2_defaults(),
            },
            None,
        )
        .await?;
    let (surface, surface_config) = {
        let surface = unsafe { instance.create_surface(&window) };

        let format = surface.get_supported_formats(&adapter)[0];
        let size = window.inner_size();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };

        surface.configure(&device, &surface_config);

        (surface, surface_config)
    };

    let mut renderer = Renderer {
        device,
        queue,
        surface,
        surface_config,
    };

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
