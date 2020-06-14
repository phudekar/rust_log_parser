use super::log_message::{LogMessage, LogMessageParser, MessageType};

#[derive(Debug)]
pub struct ErrorMessage;

impl LogMessageParser for ErrorMessage {
    fn parse(input: &str) -> Option<LogMessage> {
        let mut iter = input.splitn(3, " ");
        let error_code_result = iter.next().unwrap_or_default().parse();
        let timestamp_result = iter.next().unwrap_or_default().parse();

        if input.split_whitespace().count() < 3
            || timestamp_result.is_err()
            || error_code_result.is_err()
        {
            return None;
        } else {
            let message: String = String::from(iter.next().unwrap_or_default());
            let timestamp = timestamp_result.unwrap_or_default();
            let error_code = error_code_result.unwrap_or_default();
            return Option::Some(LogMessage {
                message_type: MessageType::Error { error_code },
                timestamp,
                message,
            });
        }
    }
}

#[test]
fn should_parse_error_message() {
    let message_ops = ErrorMessage::parse("1 23 file closed");
    assert!(message_ops.is_some(), "Expected message to have a value");
    let log = message_ops.unwrap();

    assert_eq!(log.message, "file closed");
    assert_eq!(log.message_type, MessageType::Error { error_code: 1 });
    assert_eq!(log.timestamp, 23);
}

#[test]
fn should_not_parse_error_message_if_does_not_have_timestamp() {
    let message = ErrorMessage::parse("checking things");
    assert!(message.is_none(), "Expected message to be empty");
}

#[test]
fn should_not_parse_error_message_if_does_not_have_errorcode() {
    let message = ErrorMessage::parse("23 checking things");
    assert!(message.is_none(), "Expected message to be empty");
}
