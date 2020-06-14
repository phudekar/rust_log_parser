#[derive(Debug)]
pub struct UnknownMessage {
    pub message: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum MessageType {
    Info,
    Warning,
    Error { error_code: i32 },
}

#[derive(Debug, PartialEq, Clone)]
pub struct LogMessage {
    pub message_type: MessageType,
    pub timestamp: u32,
    pub message: String,
}

pub trait LogMessageParser {
    fn parse(input: &str) -> Option<LogMessage>;
}
