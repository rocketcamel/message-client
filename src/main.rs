use std::{sync, time::Duration};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use tokio::sync::{self, mpsc};
use tui::{
    Terminal,
    backend::CrosstermBackend,
    widgets::{Block, Borders},
};

mod input;

pub struct AppState {
    reqwest_client: reqwest::Client,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            reqwest_client: reqwest::Client::new(),
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app_state = AppState::new();

    let (input_tx, input) = mpsc::unbounded_channel::<KeyCode>();
    let (state_tx, state_rx) = mpsc::unbounded_channel::<AppState>();

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal: Terminal<CrosstermBackend<std::io::Stdout>> = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().title("Block").borders(Borders::ALL);
        f.render_widget(block, size);
    })?;

    std::thread::sleep(Duration::from_secs(5));
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    Ok(())
}
