use std::num::NonZeroU32;

use crossbeam_channel::{SendError, Sender};
use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use crate::synth::input::{Input, Note};

pub fn run(input: Sender<Input>) {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface =
        unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();

    event_loop.run(move |event, _, control_flow| {
        let event = match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    match input.virtual_keycode {
                        Some(VirtualKeyCode::Escape) => {
                            *control_flow = ControlFlow::Exit;
                            return;
                        }

                        Some(VirtualKeyCode::Left) => Input::OctaveDec,
                        Some(VirtualKeyCode::Right) => Input::OctaveInc,

                        Some(VirtualKeyCode::Down) => Input::VolumeDec,
                        Some(VirtualKeyCode::Up) => Input::VolumeInc,

                        Some(VirtualKeyCode::A) => Input::PlayNote(Note::C),
                        Some(VirtualKeyCode::S) => Input::PlayNote(Note::D),
                        Some(VirtualKeyCode::D) => Input::PlayNote(Note::E),
                        Some(VirtualKeyCode::F) => Input::PlayNote(Note::F),
                        Some(VirtualKeyCode::G) => Input::PlayNote(Note::G),
                        Some(VirtualKeyCode::H) => Input::PlayNote(Note::A),
                        Some(VirtualKeyCode::J) => Input::PlayNote(Note::B),

                        _ => return,
                    }
                }
                _ => return,
            },
            Event::RedrawRequested(_) => {
                let size = window.inner_size();
                surface
                    .resize(
                        NonZeroU32::new(size.width).unwrap(),
                        NonZeroU32::new(size.height).unwrap(),
                    )
                    .unwrap();
                let buffer = surface.buffer_mut().unwrap();
                buffer.present().unwrap();
                return;
            }
            _ => return,
        };

        match input.send(event) {
            Ok(()) => {}
            Err(SendError(_)) => {
                // channel is disconnected
                *control_flow = ControlFlow::Exit;
            }
        }
    });
}
