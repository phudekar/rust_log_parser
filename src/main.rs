mod messages;
mod parser;

use messages::log_message::{LogMessage, MessageType};
use messages::tree::message_tree::MessageTree;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_severity = String::from("50");
    match args.get(1) {
        Some(input_file) => match File::open(input_file) {
            Ok(file) => {
                let lines = BufReader::new(file).lines();
                let mut messages: Vec<LogMessage> = vec![];
                for line in lines {
                    if let Ok(result) = line {
                        match parser::parse_message(result.as_str()) {
                            Some(log_message) => messages.push(log_message),
                            _ => (),
                        }
                    }
                }
                if let Ok(severity) = args.get(2).unwrap_or(&default_severity).parse() {
                    if let Ok(errors) = what_went_wrong(&messages, severity) {
                        for error in errors {
                            println!("{}", error);
                        }
                    } else {
                        println!("Failed to process messages.")
                    }
                } else {
                    println!("Please enter correct integer value for severity.")
                }
            }
            Err(error) => println!("{} {}", error, args.get(1).unwrap()),
        },
        _ => println!("Please provide log file as input argument"),
    }
}

// fn parse_line(line: String) {
//     match parser::parse_message(line.as_str()) {
//         Some(log_message) => println!("{}", log_message),
//         _ => println!(
//             "{}",
//             UnknownMessage {
//                 message: String::from(line)
//             }
//         ),
//     }
// }

fn what_went_wrong(messages: &Vec<LogMessage>, sevierity: i32) -> Result<Vec<LogMessage>, String> {
    let errors: Vec<LogMessage> = MessageTree::build(messages)?
        .in_order()
        .into_iter()
        .filter(|message| match message.message_type {
            MessageType::Error { error_code } => error_code >= sevierity,
            _ => false,
        })
        .map(|error| error.clone())
        .collect();
    Ok(errors)
}
