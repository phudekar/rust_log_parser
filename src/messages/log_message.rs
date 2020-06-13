#[derive(Debug)]
pub struct UnknownMessage {
    pub message: String,
}

#[derive(Debug, PartialEq)]
pub enum MessageType {
    Info,
    Warning,
    Error { error_code: u32 },
}

#[derive(Debug, PartialEq)]
pub struct LogMessage {
    pub message_type: MessageType,
    pub timestamp: u32,
    pub message: String,
}

pub trait LogMessageParser {
    fn parse(input: &str) -> Option<LogMessage>;
}
