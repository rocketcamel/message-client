use crossterm::event::{Event, EventStream, KeyCode, KeyEvent};
use futures::StreamExt;
use tokio::sync::mpsc;

pub enum InputEvent {
    Quit,
    Key(KeyEvent),
}

pub async fn handle_input(
    input_tx: mpsc::UnboundedSender<InputEvent>,
) -> mpsc::UnboundedReceiver<InputEvent> {
    tokio::spawn(async move {
        let mut event_stream = EventStream::new();

        loop {
            if let Some(Ok(Event::Key(key))) = event_stream.next().await {
                match key.code {
                    KeyCode::Char('q') => {}
                }
            }
        }
    })
}
