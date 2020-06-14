use std::fmt;

use super::log_message::LogMessage;
use super::log_message::MessageType;
use super::log_message::UnknownMessage;
use super::message_tree::MessageTree;
use super::message_tree::TreeNode;

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
            "LogMessage {} {} {}",
            self.message_type, self.timestamp, self.message
        )
    }
}

impl std::fmt::Display for MessageTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.node_type {
            TreeNode::Leaf => writeln!(f, "{}", self.message.clone()),
            TreeNode::Node { left, right } => {
                let _ = writeln!(f, "{}", self.message.clone());
                let _ = left.clone().map(|s| writeln!(f, "-- {}", &s));
                let _ = right.clone().map(|s| writeln!(f, "-- {}", &s));
                writeln!(f, "|")
            }
        }
    }
}
