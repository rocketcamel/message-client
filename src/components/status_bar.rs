use std::{cell::RefCell, rc::Rc};

use tui::{
    Frame,
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::Paragraph,
};

use crate::state::AppState;

#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
    Connecting,
}

pub struct StatusBar {
    app_state: Rc<RefCell<AppState>>,
}

impl StatusBar {
    pub fn new(app_state: Rc<RefCell<AppState>>) -> Self {
        Self { app_state }
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let state = self.app_state.borrow();
        let (status_text, status_color) = match &state.connection_status {
            ConnectionStatus::Connected => ("Connected", Color::Green),
            ConnectionStatus::Disconnected => ("Disconnected", Color::Red),
            ConnectionStatus::Connecting => ("Connecting...", Color::Yellow),
        };

        let spans = Spans::from(vec![
            Span::styled(
                format!(" {} ", status_text),
                Style::default()
                    .fg(status_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" | ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{} messages", state.messages.len()),
                Style::default().fg(Color::White),
            ),
            Span::styled(" | ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "[↑/↓]",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Scroll ", Style::default().fg(Color::White)),
            Span::styled(
                "[Enter]",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Send ", Style::default().fg(Color::White)),
            Span::styled(
                "[Esc]",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Clear ", Style::default().fg(Color::White)),
            Span::styled(
                "[Ctrl+S]",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Config ", Style::default().fg(Color::White)),
            Span::styled(
                "[Ctrl+C]",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Quit ", Style::default().fg(Color::White)),
        ]);

        let paragraph = Paragraph::new(spans).style(Style::default().fg(Color::White));

        f.render_widget(paragraph, area);
    }
}
