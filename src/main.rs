use std::{cell::RefCell, rc::Rc, time::Duration};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use tokio::sync::mpsc::{self, error::TryRecvError};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
use tui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders},
};

use crate::{
    components::{
        Config, ConnectionStatus, InputBox, Message, MessageList, MessageSender, StatusBar,
    },
    network::AuthRequest,
    state::{AppState, FocusedItem},
};
use crate::{
    input::InputEvent,
    network::{NetworkRequest, NetworkResponse},
};

mod components;
mod input;
mod network;
mod state;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app_state = Rc::new(RefCell::new(AppState::new()));

    let (input_tx, mut input_rx) = mpsc::unbounded_channel::<InputEvent>();
    let (req_tx, req_rx) = mpsc::unbounded_channel::<NetworkRequest>();
    let (resp_tx, mut resp_rx) = mpsc::unbounded_channel::<NetworkResponse>();

    let tracing_env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(tracing_env_filter)
        .init();

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal: Terminal<CrosstermBackend<std::io::Stdout>> = Terminal::new(backend)?;

    tokio::spawn(input::handle_input(input_tx));
    tokio::spawn(async move {
        network::NetworkTask::new().run(req_rx, resp_tx).await;
    });
    // if let Err(e) = req_tx.send(NetworkRequest::Authenticate(AuthRequest {
    //     name: "example".to_string(),
    //     password: std::env::var("PASSWORD").expect("\"PASSWORD\" environment variable"),
    // })) {
    //     tracing::error!("error sending authentication request: {e}")
    // }

    let message_list = MessageList::new(app_state.clone());
    let input_box = InputBox::new(app_state.clone());
    let status_bar = StatusBar::new(app_state.clone());
    let mut config = Config::new(app_state.clone());

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Title bar
                    Constraint::Min(10),   // Message history
                    Constraint::Length(3), // Input box
                    Constraint::Length(1), // Status bar
                ])
                .split(size);

            let title_block = Block::default()
                .title("Message Client")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue));
            f.render_widget(title_block, chunks[0]);

            config.render(f);
            message_list.render(f, chunks[1]);
            input_box.render(f, chunks[2]);
            status_bar.render(f, chunks[3]);
        })?;

        let mut app_state = app_state.borrow_mut();
        match input_rx.try_recv() {
            Ok(InputEvent::Quit) => {
                break;
            }
            Ok(InputEvent::Submit) => app_state.send_message(),
            Ok(InputEvent::CharInput(c)) => match app_state.focused_item {
                FocusedItem::Main => app_state.insert_char(c),
                FocusedItem::Config => config.insert_char(c),
            },
            Ok(InputEvent::Backspace) => match app_state.focused_item {
                FocusedItem::Main => app_state.backspace(),
                FocusedItem::Config => config.backspace(),
            },
            Ok(InputEvent::Delete) => match app_state.focused_item {
                FocusedItem::Main => app_state.delete_char(),
                FocusedItem::Config => config.delete_char(),
            },
            Ok(InputEvent::CursorLeft) => match app_state.focused_item {
                FocusedItem::Main => app_state.move_cursor_left(),
                FocusedItem::Config => config.move_cursor_left(),
            },
            Ok(InputEvent::CursorRight) => match app_state.focused_item {
                FocusedItem::Main => app_state.move_cursor_right(),
                FocusedItem::Config => config.move_cursor_right(),
            },
            Ok(InputEvent::ScrollUp) => app_state.scroll_up(),
            Ok(InputEvent::ScrollDown) => app_state.scroll_down(),
            Ok(InputEvent::Esc) => match app_state.focused_item {
                FocusedItem::Main => app_state.clear_input(),
                FocusedItem::Config => {
                    config.close();
                    app_state.focused_item = FocusedItem::Main;
                }
            },
            Ok(InputEvent::OpenConfig) => {
                app_state.focused_item = FocusedItem::Config;
                config.open();
            }
            Ok(InputEvent::NextField) => match app_state.focused_item {
                FocusedItem::Main => {}
                FocusedItem::Config => config.next_field(),
            },
            Ok(InputEvent::PrevField) => match app_state.focused_item {
                FocusedItem::Main => {}
                FocusedItem::Config => config.previous_field(),
            },
            Err(mpsc::error::TryRecvError::Empty) => {}
            Err(mpsc::error::TryRecvError::Disconnected) => {
                break;
            }
        }

        match resp_rx.try_recv() {
            Ok(NetworkResponse::AuthSuccess { token }) => app_state.update_session(Some(token)),
            Ok(NetworkResponse::AuthError { error }) => {
                tracing::error!("error authenticating: {error}")
            }
            Ok(_) => {}
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {}
        }

        // tokio::time::sleep(Duration::from_micros(1)).await;
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    Ok(())
}
