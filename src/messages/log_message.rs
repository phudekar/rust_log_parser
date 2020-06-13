#[derive(Debug)]
pub struct UnknownMessage {
    pub message: String,
}

#[derive(Debug, PartialEq)]
pub enum MessageType {
    I,
    W,
    E,
}

pub trait LogMessage {
    fn message_type(&self) -> MessageType;
    fn timestamp(&self) -> u32;
    fn message(&self) -> String;
}

pub trait LogMessageParser<T> {
    fn parse(input: &str) -> Option<T>;
}
