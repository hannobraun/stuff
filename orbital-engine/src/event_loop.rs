use anyhow::bail;
use tokio::{fs::File, io::AsyncReadExt, process::Command};
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

    let status = Command::new("cargo")
        .arg("rustc")
        .args(["--manifest-path", "../orbital-game/Cargo.toml"])
        .args(["--target", "wasm32-unknown-unknown"])
        .args(["--crate-type", "cdylib"])
        .status()
        .await?;
    if !status.success() {
        bail!("Error building WASM module");
    }

    let mut wasm = Vec::new();
    File::open("../target/wasm32-unknown-unknown/debug/orbital_game.wasm")
        .await?
        .read_to_end(&mut wasm)
        .await?;

    let mut store = wasmer::Store::default();
    let module = wasmer::Module::new(&store, &wasm)?;
    let imports = wasmer::imports! {};
    let instance = wasmer::Instance::new(&mut store, &module, &imports)?;

    let mut host = Host { store, instance };

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
            let get_color =
                host.instance.exports.get_function("color").unwrap();
            let result = &*get_color.call(&mut host.store, &[]).unwrap();
            let &[wasmer::Value::F64(value)] = result else { panic!() };

            color = [value, value, value, 1.];
            window.request_redraw();
        }
        Event::RedrawRequested(_) => {
            renderer.draw(color).unwrap();
        }
        _ => {}
    })
}
