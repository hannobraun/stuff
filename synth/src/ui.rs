use std::num::NonZeroU32;

use crossbeam_channel::{SendError, Sender};
use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
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
                    match (input.virtual_keycode, input.state) {
                        (Some(VirtualKeyCode::Escape), _) => {
                            *control_flow = ControlFlow::Exit;
                            return;
                        }

                        (Some(VirtualKeyCode::Left), ElementState::Pressed) => {
                            Input::OctaveDec
                        }
                        (
                            Some(VirtualKeyCode::Right),
                            ElementState::Pressed,
                        ) => Input::OctaveInc,

                        (Some(VirtualKeyCode::Down), ElementState::Pressed) => {
                            Input::VolumeDec
                        }
                        (Some(VirtualKeyCode::Up), ElementState::Pressed) => {
                            Input::VolumeInc
                        }

                        (Some(VirtualKeyCode::A), ElementState::Pressed) => {
                            Input::PlayNote(Note::C)
                        }
                        (Some(VirtualKeyCode::S), ElementState::Pressed) => {
                            Input::PlayNote(Note::D)
                        }
                        (Some(VirtualKeyCode::D), ElementState::Pressed) => {
                            Input::PlayNote(Note::E)
                        }
                        (Some(VirtualKeyCode::F), ElementState::Pressed) => {
                            Input::PlayNote(Note::F)
                        }
                        (Some(VirtualKeyCode::G), ElementState::Pressed) => {
                            Input::PlayNote(Note::G)
                        }
                        (Some(VirtualKeyCode::H), ElementState::Pressed) => {
                            Input::PlayNote(Note::A)
                        }
                        (Some(VirtualKeyCode::J), ElementState::Pressed) => {
                            Input::PlayNote(Note::B)
                        }

                        (Some(VirtualKeyCode::A), ElementState::Released) => {
                            Input::ReleaseNote
                        }
                        (Some(VirtualKeyCode::S), ElementState::Released) => {
                            Input::ReleaseNote
                        }
                        (Some(VirtualKeyCode::D), ElementState::Released) => {
                            Input::ReleaseNote
                        }
                        (Some(VirtualKeyCode::F), ElementState::Released) => {
                            Input::ReleaseNote
                        }
                        (Some(VirtualKeyCode::G), ElementState::Released) => {
                            Input::ReleaseNote
                        }
                        (Some(VirtualKeyCode::H), ElementState::Released) => {
                            Input::ReleaseNote
                        }
                        (Some(VirtualKeyCode::J), ElementState::Released) => {
                            Input::ReleaseNote
                        }

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
