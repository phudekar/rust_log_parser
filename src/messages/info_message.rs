use super::log_message::LogMessage;
use super::log_message::LogMessageParser;
use super::log_message::MessageType;

#[derive(Debug)]
pub struct InfoMessage;

pub struct WarningMessage;

impl LogMessageParser for InfoMessage {
    fn parse(input: &str) -> Option<LogMessage> {
        parse_log_message(input, MessageType::Info)
    }
}

impl LogMessageParser for WarningMessage {
    fn parse(input: &str) -> Option<LogMessage> {
        parse_log_message(input, MessageType::Warning)
    }
}

fn parse_log_message(input: &str, message_type: MessageType) -> Option<LogMessage> {
    let mut iter = input.split_whitespace();
    let timestamp_result = iter.next().unwrap_or_default().parse();

    if input.split_whitespace().count() < 2 || timestamp_result.is_err() {
        return Option::None;
    } else {
        let words: Vec<&str> = iter.collect();
        let message: String = words.join(" ");
        let timestamp = timestamp_result.unwrap_or_default();
        return Option::Some(LogMessage {
            message_type,
            timestamp,
            message,
        });
    }
}

#[test]
fn should_parse_info_message() {
    let message_ops = InfoMessage::parse("23 checking things");
    assert!(message_ops.is_some(), "Expected message to have a value");
    let log = message_ops.unwrap();
    assert_eq!(log.message_type, MessageType::Info);
    assert_eq!(log.message, "checking things");
    assert_eq!(log.timestamp, 23);
}

#[test]
fn should_not_parse_info_message_if_does_not_have_timestamp() {
    let message = InfoMessage::parse("checking things");
    assert!(message.is_none(), "Expected message to be empty");
}
