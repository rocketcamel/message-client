use std::{borrow::Cow, sync::Arc};

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Message {
    pub timestamp: DateTime<Utc>,
    pub sender: MessageSender,
    pub content: String,
    pub username: Option<Arc<str>>,
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

    pub fn sender_name(&self) -> Cow<'_, str> {
        match self.sender {
            MessageSender::User(id) => self
                .username
                .as_ref()
                .map(|arc| Cow::Borrowed(arc.as_ref()))
                .unwrap_or_else(|| Cow::Owned(format!("User: {id}"))),
            MessageSender::System => Cow::Borrowed("System"),
        }
    }
}
