use std::{sync::Arc, time::Duration};

use chrono::{DateTime, Utc};

use crate::{
    components::{ConnectionStatus, Message, MessageSender},
    network::Token,
};

pub enum FocusedItem {
    Main,
    Config,
}

pub struct AppState {
    pub messages: Vec<Message>,
    pub input_buffer: String,
    pub cursor_position: usize,
    pub scroll_offset: u16,
    pub connection_status: ConnectionStatus,
    pub focused_item: FocusedItem,
    pub session_token: Option<Arc<Token>>,
    pub last_reconnect: Option<tokio::time::Instant>,
    pub reconnect_duration: Duration,
}

impl AppState {
    pub fn new() -> Self {
        let mut messages = Vec::new();

        messages.push(Message {
            sender: MessageSender::System,
            content: "Welcome to Message Client! Start typing to send messages.".to_string(),
            timestamp: Utc::now(),
            username: None,
        });

        Self {
            messages,
            input_buffer: String::new(),
            cursor_position: 0,
            scroll_offset: 0,
            connection_status: ConnectionStatus::Disconnected,
            focused_item: FocusedItem::Main,
            session_token: None,
            last_reconnect: None,
            reconnect_duration: Duration::from_secs(5),
        }
    }

    pub fn update_session(&mut self, token: Option<Arc<Token>>) {
        if token.is_some() {
            self.connection_status = ConnectionStatus::Connected;
        }
        self.session_token = token;
    }

    pub fn add_message(
        &mut self,
        sender: MessageSender,
        content: String,
        timestamp: DateTime<Utc>,
        username: Option<Arc<str>>,
    ) {
        self.messages.insert(
            0,
            Message {
                sender,
                content,
                timestamp,
                username,
            },
        );
    }

    pub fn send_message(&mut self) -> Option<String> {
        if !self.input_buffer.trim().is_empty()
            && let Some(token) = &self.session_token
        {
            let message = self.input_buffer.clone();
            self.add_message(
                MessageSender::User(token.user_id),
                message.clone(),
                Utc::now(),
                token.username.clone(),
            );
            self.input_buffer.clear();
            self.cursor_position = 0;
            return Some(message.clone());
        }
        None
    }

    pub fn insert_char(&mut self, c: char) {
        self.input_buffer.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    pub fn delete_char(&mut self) {
        if self.cursor_position < self.input_buffer.len() {
            self.input_buffer.remove(self.cursor_position);
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.input_buffer.remove(self.cursor_position);
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.input_buffer.len() {
            self.cursor_position += 1;
        }
    }

    pub fn clear_input(&mut self) {
        self.input_buffer.clear();
        self.cursor_position = 0;
    }

    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_add(1);
    }
}
