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
    username: String,
    password: String,
    server_url: String,
    focused_field: ConfigField,
    is_visible: bool,
}

impl Config {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
            server_url: "http://ec2-44-250-68-143.us-west-2.compute.amazonaws.com:8000".to_string(),
            focused_field: ConfigField::Username,
            is_visible: false,
        }
    }

    pub fn next_field(&mut self) {
        self.focused_field = match self.focused_field {
            ConfigField::Username => ConfigField::Password,
            ConfigField::Password => ConfigField::ServerUrl,
            ConfigField::ServerUrl => ConfigField::Username,
        };
    }

    pub fn previous_field(&mut self) {
        self.focused_field = match self.focused_field {
            ConfigField::Username => ConfigField::ServerUrl,
            ConfigField::Password => ConfigField::Username,
            ConfigField::ServerUrl => ConfigField::Password,
        };
    }

    pub fn insert_char(&mut self, c: char) {
        match self.focused_field {
            ConfigField::Username => self.username.push(c),
            ConfigField::Password => self.password.push(c),
            ConfigField::ServerUrl => self.server_url.push(c),
        }
    }

    pub fn delete_char(&mut self) {
        match self.focused_field {
            ConfigField::Username => {
                self.username.pop();
            }
            ConfigField::Password => {
                self.password.pop();
            }
            ConfigField::ServerUrl => {
                self.server_url.pop();
            }
        }
    }

    pub fn clear_field(&mut self) {
        match self.focused_field {
            ConfigField::Username => self.username.clear(),
            ConfigField::Password => self.password.clear(),
            ConfigField::ServerUrl => self.server_url.clear(),
        }
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
        );

        self.render_field(
            f,
            chunks[1],
            "Password",
            &self.password,
            ConfigField::Password,
            true,
        );

        self.render_field(
            f,
            chunks[2],
            "Server URL",
            &self.server_url,
            ConfigField::ServerUrl,
            false,
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

        let text = if display_value.is_empty() {
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
