extern crate either;
use crate::messages::error_message::ErrorMessage;
use crate::messages::info_message::InfoMessage;
use crate::messages::info_message::WarningMessage;

use crate::messages::log_message::LogMessageParser;
use crate::messages::log_message::*;

pub fn parse_message(input: &str) -> Option<LogMessage> {
    return get_message_type(input)
        .map(|messag_type| {
            let message: &str = &input[2..];
            match messag_type {
                MessageType::Info => InfoMessage::parse(message),
                MessageType::Warning => WarningMessage::parse(message),
                _ => ErrorMessage::parse(message),
            }
        })
        .flatten();
}

fn get_message_type(input: &str) -> Option<MessageType> {
    return match &input[0..2] {
        "I " => Some(MessageType::Info),
        "W " => Some(MessageType::Warning),
        "E " => Some(MessageType::Error { error_code: 0 }),
        _ => None,
    };
}

#[test]
fn should_parse_info_message_type() {
    let message_type = get_message_type("I 23 checking things");
    assert!(message_type.is_some(), "Expected message to have a value");
    assert_eq!(message_type.unwrap(), MessageType::Info)
}

#[test]
fn should_parse_waring_message_type() {
    let message_type = get_message_type("W 23 checking things");
    assert!(message_type.is_some(), "Expected message to have a value");
    assert_eq!(message_type.unwrap(), MessageType::Warning)
}

#[test]
fn should_parse_error_message_type() {
    let message_type = get_message_type("E 12 23 checking things");
    assert!(message_type.is_some(), "Expected message to have a value");
    assert_eq!(message_type.unwrap(), MessageType::Error { error_code: 0 })
}

#[test]
fn should_parse_info_message() {
    let message_ops = parse_message("I 23 checking things");
    assert!(message_ops.is_some(), "Expected message to have a value");
    let message = message_ops.unwrap();
    assert_eq!(message.message_type, MessageType::Info);
    assert_eq!(message.message, "checking things");
    assert_eq!(message.timestamp, 23);
}
