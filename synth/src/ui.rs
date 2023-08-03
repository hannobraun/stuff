use std::num::NonZeroU32;

use crossbeam_channel::{SendError, Sender};
use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::KeyCode,
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
                WindowEvent::KeyboardInput { event, .. } => {
                    match (event.physical_key, event.state) {
                        (KeyCode::Escape, _) => {
                            *control_flow = ControlFlow::Exit;
                            return;
                        }

                        (KeyCode::ArrowLeft, ElementState::Pressed) => {
                            UserInput::OctaveDec
                        }
                        (KeyCode::ArrowRight, ElementState::Pressed) => {
                            UserInput::OctaveInc
                        }

                        (KeyCode::ArrowDown, ElementState::Pressed) => {
                            UserInput::VolumeDec
                        }
                        (KeyCode::ArrowUp, ElementState::Pressed) => {
                            UserInput::VolumeInc
                        }

                        (KeyCode::KeyA, ElementState::Pressed) => {
                            UserInput::PlayNote(Note::C)
                        }
                        (KeyCode::KeyS, ElementState::Pressed) => {
                            UserInput::PlayNote(Note::D)
                        }
                        (KeyCode::KeyD, ElementState::Pressed) => {
                            UserInput::PlayNote(Note::E)
                        }
                        (KeyCode::KeyF, ElementState::Pressed) => {
                            UserInput::PlayNote(Note::F)
                        }
                        (KeyCode::KeyG, ElementState::Pressed) => {
                            UserInput::PlayNote(Note::G)
                        }
                        (KeyCode::KeyH, ElementState::Pressed) => {
                            UserInput::PlayNote(Note::A)
                        }
                        (KeyCode::KeyJ, ElementState::Pressed) => {
                            UserInput::PlayNote(Note::B)
                        }

                        (KeyCode::KeyA, ElementState::Released) => {
                            UserInput::ReleaseNote(Note::C)
                        }
                        (KeyCode::KeyS, ElementState::Released) => {
                            UserInput::ReleaseNote(Note::D)
                        }
                        (KeyCode::KeyD, ElementState::Released) => {
                            UserInput::ReleaseNote(Note::E)
                        }
                        (KeyCode::KeyF, ElementState::Released) => {
                            UserInput::ReleaseNote(Note::F)
                        }
                        (KeyCode::KeyG, ElementState::Released) => {
                            UserInput::ReleaseNote(Note::G)
                        }
                        (KeyCode::KeyH, ElementState::Released) => {
                            UserInput::ReleaseNote(Note::A)
                        }
                        (KeyCode::KeyJ, ElementState::Released) => {
                            UserInput::ReleaseNote(Note::B)
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
