use std::{cell::RefCell, rc::Rc};

use tui::{
    Frame,
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

use crate::state::AppState;

pub struct InputBox {
    is_focused: bool,
    app_state: Rc<RefCell<AppState>>,
}

impl InputBox {
    pub fn new(app_state: Rc<RefCell<AppState>>) -> Self {
        Self {
            app_state,
            is_focused: true,
        }
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let state = self.app_state.borrow();
        let border_style = if self.is_focused {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let input_text = if state.input_buffer.is_empty() {
            Spans::from(vec![Span::styled(
                "Type your message here...",
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::ITALIC),
            )])
        } else {
            let before_cursor =
                &state.input_buffer[..state.cursor_position.min(state.input_buffer.len())];
            let after_cursor =
                &state.input_buffer[state.cursor_position.min(state.input_buffer.len())..];

            let cursor_char = if state.cursor_position >= state.input_buffer.len() {
                " "
            } else {
                &state.input_buffer[state.cursor_position..state.cursor_position + 1]
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

        let char_count = format!(" {}/{} ", state.input_buffer.len(), 500);
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
