use std::thread;

use crossbeam_channel::{Receiver, SendError};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};

pub fn start() -> Receiver<UiEvent> {
    let (tx, rx) = crossbeam_channel::bounded(0);

    thread::spawn(move || {
        loop {
            let event = match read_event() {
                Ok(Some(event)) => event,
                Ok(None) => continue,
                Err(err) => panic!("{err}"),
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

    rx
}

fn read_event() -> anyhow::Result<Option<UiEvent>> {
    if let Event::Key(key) = event::read()? {
        if key.code == KeyCode::Char('c')
            && key.modifiers.contains(KeyModifiers::CONTROL)
        {
            return Ok(Some(UiEvent::Quit));
        }

        if key.code == KeyCode::Left {
            return Ok(Some(UiEvent::OctaveDec));
        }
        if key.code == KeyCode::Right {
            return Ok(Some(UiEvent::OctaveInc));
        }

        if key.code == KeyCode::Down {
            return Ok(Some(UiEvent::VolumeDec));
        }
        if key.code == KeyCode::Up {
            return Ok(Some(UiEvent::VolumeInc));
        }

        if let KeyCode::Char(c) = key.code {
            if c == 'a' {
                return Ok(Some(UiEvent::PlayNote(Note::C)));
            }
            if c == 's' {
                return Ok(Some(UiEvent::PlayNote(Note::D)));
            }
            if c == 'd' {
                return Ok(Some(UiEvent::PlayNote(Note::E)));
            }
            if c == 'f' {
                return Ok(Some(UiEvent::PlayNote(Note::F)));
            }
            if c == 'g' {
                return Ok(Some(UiEvent::PlayNote(Note::G)));
            }
            if c == 'h' {
                return Ok(Some(UiEvent::PlayNote(Note::A)));
            }
            if c == 'j' {
                return Ok(Some(UiEvent::PlayNote(Note::B)));
            }
        }

        dbg!(key.code);
    }

    Ok(None)
}

pub enum UiEvent {
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
