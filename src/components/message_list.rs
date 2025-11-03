use tui::{
    Frame,
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use super::message::{Message, MessageSender};

pub struct MessageList<'a> {
    messages: &'a [Message],
    scroll_offset: u16,
}

impl<'a> MessageList<'a> {
    pub fn new(messages: &'a [Message], scroll_offset: u16) -> Self {
        Self {
            messages,
            scroll_offset,
        }
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let mut text_lines = Vec::new();

        for message in self.messages {
            let timestamp_style = Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::DIM);

            let (sender_style, content_style) = match message.sender {
                MessageSender::User => (
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                    Style::default().fg(Color::White),
                ),
                MessageSender::Server => (
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                    Style::default().fg(Color::White),
                ),
                MessageSender::System => (
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::ITALIC),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::DIM),
                ),
            };

            let line = Spans::from(vec![
                Span::styled(format!("[{}] ", message.format_time()), timestamp_style),
                Span::styled(format!("{}: ", message.sender_name()), sender_style),
                Span::styled(message.content.clone(), content_style),
            ]);

            text_lines.push(line);
        }

        let text = Text::from(text_lines);

        let block = Block::default()
            .title("Messages")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue));

        let paragraph = Paragraph::new(text)
            .block(block)
            .wrap(Wrap { trim: false })
            .scroll((self.scroll_offset, 0));

        f.render_widget(paragraph, area);
    }
}
