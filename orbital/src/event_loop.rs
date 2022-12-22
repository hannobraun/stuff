use anyhow::anyhow;
use winit::{event::Event, event_loop::EventLoop, window::Window};

pub async fn run() -> anyhow::Result<()> {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop)?;

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
    let surface = {
        let surface = unsafe { instance.create_surface(&window) };

        let format = surface.get_supported_formats(&adapter)[0];
        let size = window.inner_size();

        surface.configure(
            &device,
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format,
                width: size.width,
                height: size.height,
                present_mode: wgpu::PresentMode::AutoVsync,
                alpha_mode: wgpu::CompositeAlphaMode::Auto,
            },
        );

        surface
    };

    event_loop.run(move |event, _, _| match event {
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        Event::RedrawRequested(_) => {
            let mut encoder = device.create_command_encoder(
                &wgpu::CommandEncoderDescriptor { label: None },
            );
            let surface_texture = surface.get_current_texture().unwrap();
            let view = surface_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.,
                            g: 0.,
                            b: 0.,
                            a: 1.,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            queue.submit([encoder.finish()]);
            surface_texture.present();
        }
        _ => {
            dbg!(event);
        }
    })
}
