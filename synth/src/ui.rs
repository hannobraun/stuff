use crossbeam_channel::{SendError, Sender};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};

use crate::synth::input::{Input, Note};

pub fn start(input: Sender<Input>) {
    loop {
        let event = {
            match event::read().unwrap() {
                Event::Key(key) => match key.code {
                    KeyCode::Char('c') => {
                        if key.modifiers.contains(KeyModifiers::CONTROL) {
                            break;
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

        match input.send(event) {
            Ok(()) => {}
            Err(SendError(_)) => {
                // channel is disconnected
                break;
            }
        }
    }
}
