use super::log_message::LogMessage;
use super::log_message::LogMessageParser;
use super::log_message::MessageType;

#[derive(Debug)]
pub struct InfoMessage {
    pub timestamp: u32,
    pub message: String,
}

impl LogMessage for InfoMessage {
    fn message_type(&self) -> MessageType {
        MessageType::I
    }

    fn timestamp(&self) -> u32 {
        self.timestamp
    }

    fn message(&self) -> String {
        self.message.clone()
    }
}

impl LogMessageParser<InfoMessage> for InfoMessage {
    fn parse(input: &str) -> Option<InfoMessage> {
        let mut iter = input.split_whitespace();
        let timestamp_result = iter.next().unwrap_or_default().parse();

        if input.split_whitespace().count() < 2 || timestamp_result.is_err() {
            return Option::None;
        } else {
            let words: Vec<&str> = iter.collect();
            let message: String = words.join(" ");
            let timestamp = timestamp_result.unwrap_or_default();
            return Option::Some(InfoMessage { timestamp, message });
        }
    }
}

#[test]
fn should_parse_info_message() {
    let message_ops = InfoMessage::parse("23 checking things");
    assert!(message_ops.is_some(), "Expected message to have a value");
    let message = message_ops.unwrap();
    assert_eq!(message.message_type(), MessageType::I)
}

#[test]
fn should_not_parse_info_message_if_does_not_have_timestamp() {
    let message = InfoMessage::parse("checking things");
    assert!(message.is_none(), "Expected message to be empty");
}
