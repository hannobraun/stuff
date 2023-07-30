use crossterm::event::{self, Event, KeyCode, KeyModifiers};

pub fn read_event() -> anyhow::Result<Option<UiEvent>> {
    if let Event::Key(key) = event::read()? {
        if key.code == KeyCode::Char('c')
            && key.modifiers.contains(KeyModifiers::CONTROL)
        {
            return Ok(Some(UiEvent::Quit));
        }

        if key.code == KeyCode::Left {
            return Ok(Some(UiEvent::FrequencyDec));
        }
        if key.code == KeyCode::Right {
            return Ok(Some(UiEvent::FrequencyInc));
        }

        dbg!(key.code);
    }

    Ok(None)
}

pub enum UiEvent {
    FrequencyDec,
    FrequencyInc,

    Quit,
}