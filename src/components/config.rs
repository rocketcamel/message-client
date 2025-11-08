use tui::{
    Frame,
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConfigField {
    Username,
    Password,
    ServerUrl,
}

pub struct Config {
    pub username: String,
    pub password: String,
    pub server_url: String,
    focused_field: ConfigField,
    cursor_position: usize,
    is_visible: bool,
}

impl Config {
    pub fn new() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            server_url: "http://ec2-44-250-68-143.us-west-2.compute.amazonaws.com:8000".to_string(),
            focused_field: ConfigField::Username,
            cursor_position: 0,
            is_visible: false,
        }
    }

    pub fn next_field(&mut self) {
        self.focused_field = match self.focused_field {
            ConfigField::Username => ConfigField::Password,
            ConfigField::Password => ConfigField::ServerUrl,
            ConfigField::ServerUrl => ConfigField::Username,
        };
        self.cursor_position = self.get_field().len();
    }

    pub fn previous_field(&mut self) {
        self.focused_field = match self.focused_field {
            ConfigField::Username => ConfigField::ServerUrl,
            ConfigField::Password => ConfigField::Username,
            ConfigField::ServerUrl => ConfigField::Password,
        };
        self.cursor_position = self.get_field().len();
    }

    pub fn insert_char(&mut self, c: char) {
        let pos = self.cursor_position;
        let field = self.get_field_mut();
        field.insert(pos, c);
        self.cursor_position += 1;
    }

    pub fn delete_char(&mut self) {
        let pos = self.cursor_position;
        let field = self.get_field_mut();
        if pos < field.len() {
            field.remove(pos);
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            let pos = self.cursor_position;
            let field = self.get_field_mut();
            field.remove(pos);
        }
    }

    pub fn move_cursor_left(&mut self) {
        self.cursor_position = self.cursor_position.saturating_sub(1);
    }

    pub fn move_cursor_right(&mut self) {
        let max_pos = self.get_field().len();
        self.cursor_position = (self.cursor_position + 1).min(max_pos);
    }

    fn get_field(&self) -> &String {
        match self.focused_field {
            ConfigField::Username => &self.username,
            ConfigField::Password => &self.password,
            ConfigField::ServerUrl => &self.server_url,
        }
    }

    fn get_field_mut(&mut self) -> &mut String {
        match self.focused_field {
            ConfigField::Username => &mut self.username,
            ConfigField::Password => &mut self.password,
            ConfigField::ServerUrl => &mut self.server_url,
        }
    }

    pub fn open(&mut self) {
        self.is_visible = true;
    }

    pub fn close(&mut self) {
        self.is_visible = false;
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>) {
        if !self.is_visible {
            return;
        }

        let size = f.size();

        let popup_width = size.width.saturating_sub(10).min(70);
        let popup_height = 15;
        let popup_x = (size.width.saturating_sub(popup_width)) / 2;
        let popup_y = (size.height.saturating_sub(popup_height)) / 2;

        let popup_area = Rect {
            x: popup_x,
            y: popup_y,
            width: popup_width,
            height: popup_height,
        };

        f.render_widget(Clear, popup_area);

        let popup_block = Block::default()
            .title(" Configuration ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .style(Style::default().bg(Color::Black));

        f.render_widget(popup_block, popup_area);

        let inner_area = Rect {
            x: popup_area.x + 2,
            y: popup_area.y + 2,
            width: popup_area.width.saturating_sub(4),
            height: popup_area.height.saturating_sub(4),
        };

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Username field
                Constraint::Length(3), // Password field
                Constraint::Length(3), // Server URL field
                Constraint::Min(1),    // Help text
            ])
            .split(inner_area);

        self.render_field(
            f,
            chunks[0],
            "Username",
            &self.username,
            ConfigField::Username,
            false,
            self.cursor_position,
        );

        self.render_field(
            f,
            chunks[1],
            "Password",
            &self.password,
            ConfigField::Password,
            true,
            self.cursor_position,
        );

        self.render_field(
            f,
            chunks[2],
            "Server URL",
            &self.server_url,
            ConfigField::ServerUrl,
            false,
            self.cursor_position,
        );

        let help_text = vec![
            Spans::from(""),
            Spans::from(vec![
                Span::styled(
                    "Tab/Shift+Tab",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(": Switch fields  "),
                Span::styled(
                    "Esc",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(": Close"),
            ]),
        ];

        let help_paragraph = Paragraph::new(help_text)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        f.render_widget(help_paragraph, chunks[3]);
    }

    fn render_field<B: Backend>(
        &self,
        f: &mut Frame<B>,
        area: Rect,
        label: &str,
        value: &str,
        field: ConfigField,
        mask: bool,
        cursor_pos: usize,
    ) {
        let is_focused = self.focused_field == field;

        let border_style = if is_focused {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let display_value = if mask {
            "*".repeat(value.len())
        } else {
            value.to_string()
        };

        let text = if is_focused {
            let chars: Vec<char> = display_value.chars().collect();
            if chars.is_empty() || cursor_pos >= chars.len() {
                let before: String = chars.iter().collect();
                Spans::from(vec![
                    Span::styled(before, Style::default().fg(Color::White)),
                    Span::styled(" ", Style::default().fg(Color::Black).bg(Color::White)),
                ])
            } else {
                let before: String = chars[..cursor_pos].iter().collect();
                let at_cursor = chars[cursor_pos];
                let after: String = chars[cursor_pos + 1..].iter().collect();
                Spans::from(vec![
                    Span::styled(before, Style::default().fg(Color::White)),
                    Span::styled(
                        at_cursor.to_string(),
                        Style::default().fg(Color::Black).bg(Color::White),
                    ),
                    Span::styled(after, Style::default().fg(Color::White)),
                ])
            }
        } else if display_value.is_empty() {
            Spans::from(vec![Span::styled(
                "...",
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::ITALIC),
            )])
        } else {
            Spans::from(vec![Span::styled(
                display_value,
                Style::default().fg(Color::White),
            )])
        };

        let title = if is_focused {
            format!(" {} (editing) ", label)
        } else {
            format!(" {} ", label)
        };

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border_style);

        let paragraph = Paragraph::new(text).block(block);

        f.render_widget(paragraph, area);
    }
}
