use tui::{
    Frame,
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

pub struct InputBox<'a> {
    input: &'a str,
    cursor_position: usize,
    is_focused: bool,
}

impl<'a> InputBox<'a> {
    pub fn new(input: &'a str, cursor_position: usize) -> Self {
        Self {
            input,
            cursor_position,
            is_focused: true,
        }
    }

    pub fn with_focus(mut self, focused: bool) -> Self {
        self.is_focused = focused;
        self
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let border_style = if self.is_focused {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let input_text = if self.input.is_empty() {
            Spans::from(vec![Span::styled(
                "Type your message here...",
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::ITALIC),
            )])
        } else {
            // Split text at cursor position to show cursor
            let before_cursor = &self.input[..self.cursor_position.min(self.input.len())];
            let after_cursor = &self.input[self.cursor_position.min(self.input.len())..];

            let cursor_char = if self.cursor_position >= self.input.len() {
                " "
            } else {
                &self.input[self.cursor_position..self.cursor_position + 1]
            };

            Spans::from(vec![
                Span::styled(before_cursor, Style::default().fg(Color::White)),
                Span::styled(
                    cursor_char,
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(after_cursor, Style::default().fg(Color::White)),
            ])
        };

        let char_count = format!(" {}/{} ", self.input.len(), 500);
        let title = if self.is_focused {
            format!("Input (Active){}", char_count)
        } else {
            format!("Input{}", char_count)
        };

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border_style);

        let paragraph = Paragraph::new(input_text).block(block);

        f.render_widget(paragraph, area);
    }
}
