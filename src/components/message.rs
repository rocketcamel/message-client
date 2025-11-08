use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Message {
    pub timestamp: DateTime<Utc>,
    pub sender: MessageSender,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageSender {
    User(u32),
    System,
}

impl Message {
    pub fn format_time(&self) -> String {
        self.timestamp.format("%H:%M:%S").to_string()
    }

    pub fn sender_name(&self) -> String {
        match self.sender {
            MessageSender::User(id) => format!("User: {id}"),
            MessageSender::System => "System".to_string(),
        }
    }
}
