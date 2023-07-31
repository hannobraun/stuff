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
                                Some(Input::Quit)
                            } else {
                                None
                            }
                        }

                        KeyCode::Left => Some(Input::OctaveDec),
                        KeyCode::Right => Some(Input::OctaveInc),

                        KeyCode::Down => Some(Input::VolumeDec),
                        KeyCode::Up => Some(Input::VolumeInc),

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

                            #[allow(clippy::manual_map)]
                            if let Some(note) = note {
                                Some(Input::PlayNote(note))
                            } else {
                                None
                            }
                        }

                        _ => None,
                    },
                    _ => None,
                }
            };

            let event = match event {
                Some(event) => event,
                None => continue,
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
