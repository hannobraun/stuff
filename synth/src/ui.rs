use std::num::NonZeroU32;

use crossbeam_channel::{SendError, Sender};
use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use crate::synth::interface::{Note, UserInput};

pub fn run(input: Sender<UserInput>) {
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
                            UserInput::OctaveDec
                        }
                        (
                            Some(VirtualKeyCode::Right),
                            ElementState::Pressed,
                        ) => UserInput::OctaveInc,

                        (Some(VirtualKeyCode::Down), ElementState::Pressed) => {
                            UserInput::VolumeDec
                        }
                        (Some(VirtualKeyCode::Up), ElementState::Pressed) => {
                            UserInput::VolumeInc
                        }

                        (Some(VirtualKeyCode::A), ElementState::Pressed) => {
                            UserInput::PlayNote(Note::C)
                        }
                        (Some(VirtualKeyCode::S), ElementState::Pressed) => {
                            UserInput::PlayNote(Note::D)
                        }
                        (Some(VirtualKeyCode::D), ElementState::Pressed) => {
                            UserInput::PlayNote(Note::E)
                        }
                        (Some(VirtualKeyCode::F), ElementState::Pressed) => {
                            UserInput::PlayNote(Note::F)
                        }
                        (Some(VirtualKeyCode::G), ElementState::Pressed) => {
                            UserInput::PlayNote(Note::G)
                        }
                        (Some(VirtualKeyCode::H), ElementState::Pressed) => {
                            UserInput::PlayNote(Note::A)
                        }
                        (Some(VirtualKeyCode::J), ElementState::Pressed) => {
                            UserInput::PlayNote(Note::B)
                        }

                        (Some(VirtualKeyCode::A), ElementState::Released) => {
                            UserInput::ReleaseNote
                        }
                        (Some(VirtualKeyCode::S), ElementState::Released) => {
                            UserInput::ReleaseNote
                        }
                        (Some(VirtualKeyCode::D), ElementState::Released) => {
                            UserInput::ReleaseNote
                        }
                        (Some(VirtualKeyCode::F), ElementState::Released) => {
                            UserInput::ReleaseNote
                        }
                        (Some(VirtualKeyCode::G), ElementState::Released) => {
                            UserInput::ReleaseNote
                        }
                        (Some(VirtualKeyCode::H), ElementState::Released) => {
                            UserInput::ReleaseNote
                        }
                        (Some(VirtualKeyCode::J), ElementState::Released) => {
                            UserInput::ReleaseNote
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
