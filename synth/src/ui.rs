use std::thread;

use crossbeam_channel::{Receiver, SendError};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};

pub fn start() -> anyhow::Result<Receiver<Input>> {
    let (tx, rx) = crossbeam_channel::bounded(0);

    thread::spawn(move || {
        loop {
            let event = read_event();

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

fn read_event() -> Option<Input> {
    if let Event::Key(key) = event::read().unwrap() {
        if key.code == KeyCode::Char('c')
            && key.modifiers.contains(KeyModifiers::CONTROL)
        {
            return Some(Input::Quit);
        }

        if key.code == KeyCode::Left {
            return Some(Input::OctaveDec);
        }
        if key.code == KeyCode::Right {
            return Some(Input::OctaveInc);
        }

        if key.code == KeyCode::Down {
            return Some(Input::VolumeDec);
        }
        if key.code == KeyCode::Up {
            return Some(Input::VolumeInc);
        }

        if let KeyCode::Char(c) = key.code {
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
                return Some(Input::PlayNote(note));
            }
        }
    }

    None
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
