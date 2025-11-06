use crossterm::event::{Event, EventStream, KeyCode, KeyModifiers};
use futures::StreamExt;
use tokio::sync::mpsc;

pub enum InputEvent {
    Quit,
    Submit,
    CharInput(char),
    Backspace,
    Delete,
    CursorLeft,
    CursorRight,
    ScrollUp,
    ScrollDown,
    Esc,
    NextField,
    PrevField,
    OpenConfig,
}

pub async fn handle_input(input_tx: mpsc::UnboundedSender<InputEvent>) {
    let mut event_stream = EventStream::new();

    loop {
        if let Some(Ok(Event::Key(key))) = event_stream.next().await {
            let event = match key.code {
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    Some(InputEvent::Quit)
                }
                KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    Some(InputEvent::OpenConfig)
                }
                KeyCode::Enter => Some(InputEvent::Submit),
                KeyCode::Char(c) => Some(InputEvent::CharInput(c)),
                KeyCode::Backspace => Some(InputEvent::Backspace),
                KeyCode::Delete => Some(InputEvent::Delete),
                KeyCode::Left => Some(InputEvent::CursorLeft),
                KeyCode::Right => Some(InputEvent::CursorRight),
                KeyCode::Up => Some(InputEvent::ScrollUp),
                KeyCode::Down => Some(InputEvent::ScrollDown),
                KeyCode::Esc => Some(InputEvent::Esc),
                KeyCode::Tab => Some(InputEvent::NextField),
                KeyCode::BackTab => Some(InputEvent::PrevField),
                _ => None,
            };

            if let Some(event) = event {
                let should_quit = matches!(event, InputEvent::Quit);
                let _ = input_tx.send(event);
                if should_quit {
                    break;
                }
            }
        }
    }
}
