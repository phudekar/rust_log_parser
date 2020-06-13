extern crate either;
use crate::messages::error_message::ErrorMessage;
use crate::messages::info_message::InfoMessage;
use crate::messages::log_message::LogMessageParser;
use crate::messages::log_message::*;

pub fn parse_line(input: &str) -> Option<Box<dyn LogMessage>> {
    return get_message_type(input)
        .map(|messag_type| {
            let message: &str = &input[2..];
            match messag_type {
                MessageType::I => InfoMessage::parse(message).map(|s| {
                    let log_message: Box<dyn LogMessage> = Box::from(s);
                    log_message
                }),

                _ => ErrorMessage::parse(message).map(|s| {
                    let log_message: Box<dyn LogMessage> = Box::from(s);
                    log_message
                }),
            }
        })
        .flatten();
}

fn get_message_type(input: &str) -> Option<MessageType> {
    return match &input[0..2] {
        "I " => Option::from(MessageType::I),
        "W " => Option::from(MessageType::W),
        "E " => Option::from(MessageType::E),
        _ => Option::None,
    };
}

#[test]
fn should_parse_info_message_type() {
    let message_type = get_message_type("I 23 checking things");
    assert!(message_type.is_some(), "Expected message to have a value");
    assert_eq!(message_type.unwrap(), MessageType::I)
}

#[test]
fn should_parse_waring_message_type() {
    let message_type = get_message_type("W 23 checking things");
    assert!(message_type.is_some(), "Expected message to have a value");
    assert_eq!(message_type.unwrap(), MessageType::W)
}

#[test]
fn should_parse_error_message_type() {
    let message_type = get_message_type("E 12 23 checking things");
    assert!(message_type.is_some(), "Expected message to have a value");
    assert_eq!(message_type.unwrap(), MessageType::E)
}

#[test]
fn should_parse_info_message() {
    let message_ops = parse_line("I 23 checking things");
    assert!(message_ops.is_some(), "Expected message to have a value");
    let message = message_ops.unwrap();
    assert_eq!(message.message_type(), MessageType::I);
    assert_eq!(message.message(), "checking things");
    assert_eq!(message.timestamp(), 23);
}
