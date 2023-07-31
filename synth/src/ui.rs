use std::thread;

use crossbeam_channel::{Receiver, SendError};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};

pub fn start() -> anyhow::Result<Receiver<Input>> {
    let (tx, rx) = crossbeam_channel::bounded(0);

    thread::spawn(move || {
        loop {
            let event = {
                match event::read().unwrap() {
                    Event::Key(key) => match key.code {
                        KeyCode::Char('c') => {
                            if key.modifiers.contains(KeyModifiers::CONTROL) {
                                Input::Quit
                            } else {
                                continue;
                            }
                        }

                        KeyCode::Left => Input::OctaveDec,
                        KeyCode::Right => Input::OctaveInc,

                        KeyCode::Down => Input::VolumeDec,
                        KeyCode::Up => Input::VolumeInc,

                        KeyCode::Char(c) => {
                            let note = match c {
                                'a' => Some(Note::C),
                                's' => Some(Note::D),
                                'd' => Some(Note::E),
                                'f' => Some(Note::F),
                                'g' => Some(Note::G),
                                'h' => Some(Note::A),
                                'j' => Some(Note::B),
                                _ => None,
                            };

                            if let Some(note) = note {
                                Input::PlayNote(note)
                            } else {
                                continue;
                            }
                        }

                        _ => continue,
                    },
                    _ => continue,
                }
            };

            match tx.send(event) {
                Ok(()) => {}
                Err(SendError(_)) => {
                    // channel is disconnected
                    break;
                }
            }
        }
    });

    Ok(rx)
}

pub enum Input {
    OctaveDec,
    OctaveInc,

    VolumeDec,
    VolumeInc,

    PlayNote(Note),

    Quit,
}

pub enum Note {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}
