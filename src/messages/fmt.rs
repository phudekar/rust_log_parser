use std::fmt;

use super::log_message::{LogMessage, MessageType, UnknownMessage};

impl std::fmt::Display for UnknownMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageType::Error { error_code } => write!(f, "(Error {})", error_code),
            _ => write!(f, "{:#?}", self),
        }
    }
}

impl std::fmt::Display for LogMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.message_type, self.timestamp, self.message
        )
    }
}
