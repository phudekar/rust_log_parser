mod messages;
mod parser;

use messages::log_message::UnknownMessage;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(input_file) => match std::fs::read_to_string(input_file) {
            Ok(contents) => {
                let lines: Vec<&str> = contents.split_terminator("\n").collect();
                let _: Vec<()> = lines.into_iter().map(parse_line).collect();
            }
            Err(error) => println!("{} {}", error, args.get(1).unwrap()),
        },
        _ => println!("Please provide log file as input argument"),
    }
}

fn parse_line(line: &str) {
    match parser::parse_message(line) {
        Some(log_message) => println!("{}", log_message),
        _ => println!(
            "{}",
            UnknownMessage {
                message: String::from(line)
            }
        ),
    }
}
