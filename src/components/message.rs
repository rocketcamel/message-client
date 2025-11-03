use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
pub struct Message {
    pub timestamp: DateTime<Local>,
    pub sender: MessageSender,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageSender {
    User,
    Server,
    System,
}

impl Message {
    pub fn new(sender: MessageSender, content: String) -> Self {
        Self {
            timestamp: Local::now(),
            sender,
            content,
        }
    }

    pub fn format_time(&self) -> String {
        self.timestamp.format("%H:%M:%S").to_string()
    }

    pub fn sender_name(&self) -> &str {
        match self.sender {
            MessageSender::User => "You",
            MessageSender::Server => "Server",
            MessageSender::System => "System",
        }
    }
}
