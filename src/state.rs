use crate::components::{ConnectionStatus, Message, MessageSender};

pub struct AppState {
    pub messages: Vec<Message>,
    pub input_buffer: String,
    pub cursor_position: usize,
    pub scroll_offset: u16,
    pub connection_status: ConnectionStatus,
}

impl AppState {
    pub fn new() -> Self {
        let mut messages = Vec::new();

        messages.push(Message::new(
            MessageSender::System,
            "Welcome to Message Client! Start typing to send messages.".to_string(),
        ));

        Self {
            messages,
            input_buffer: String::new(),
            cursor_position: 0,
            scroll_offset: 0,
            connection_status: ConnectionStatus::Disconnected,
        }
    }

    pub fn add_message(&mut self, sender: MessageSender, content: String) {
        self.messages.push(Message::new(sender, content));
    }

    pub fn send_message(&mut self) {
        if !self.input_buffer.trim().is_empty() {
            let message = self.input_buffer.clone();
            self.add_message(MessageSender::User, message);
            self.input_buffer.clear();
            self.cursor_position = 0;
        }
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
