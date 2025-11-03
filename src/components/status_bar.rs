use tui::{
    Frame,
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::Paragraph,
};

#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
    Connecting,
    Error(String),
}

pub struct StatusBar<'a> {
    connection_status: &'a ConnectionStatus,
    message_count: usize,
}

impl<'a> StatusBar<'a> {
    pub fn new(connection_status: &'a ConnectionStatus, message_count: usize) -> Self {
        Self {
            connection_status,
            message_count,
        }
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let (status_text, status_color) = match &self.connection_status {
            ConnectionStatus::Connected => ("Connected", Color::Green),
            ConnectionStatus::Disconnected => ("Disconnected", Color::Red),
            ConnectionStatus::Connecting => ("Connecting...", Color::Yellow),
            ConnectionStatus::Error(err) => (err.as_str(), Color::Red),
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
                format!("{} messages", self.message_count),
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
                "[Ctrl+C/q]",
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
