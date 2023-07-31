use std::thread;

use crossbeam_channel::{Receiver, SendError};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};

pub fn start() -> anyhow::Result<Receiver<Input>> {
    let (tx, rx) = crossbeam_channel::bounded(0);

    thread::spawn(move || {
        loop {
            let event = 'event: {
                if let Event::Key(key) = event::read().unwrap() {
                    match key.code {
                        KeyCode::Char('c') => {
                            if key.modifiers.contains(KeyModifiers::CONTROL) {
                                break 'event Some(Input::Quit);
                            }
                        }

                        KeyCode::Left => {
                            break 'event Some(Input::OctaveDec);
                        }
                        KeyCode::Right => {
                            break 'event Some(Input::OctaveInc);
                        }

                        KeyCode::Down => {
                            break 'event Some(Input::VolumeDec);
                        }
                        KeyCode::Up => {
                            break 'event Some(Input::VolumeInc);
                        }

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
                                break 'event Some(Input::PlayNote(note));
                            }
                        }

                        _ => {
                            break 'event None;
                        }
                    }
                }

                None
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
