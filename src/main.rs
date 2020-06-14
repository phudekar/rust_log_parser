mod messages;
mod parser;

use messages::log_message::UnknownMessage;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(input_file) => match File::open(input_file) {
            Ok(file) => {
                let lines = BufReader::new(file).lines();
                for line in lines {
                    if let Ok(ip) = line {
                        parse_line(ip);
                    }
                }
            }
            Err(error) => println!("{} {}", error, args.get(1).unwrap()),
        },
        _ => println!("Please provide log file as input argument"),
    }
}

fn parse_line(line: String) {
    match parser::parse_message(line.as_str()) {
        Some(log_message) => println!("{}", log_message),
        _ => println!(
            "{}",
            UnknownMessage {
                message: String::from(line)
            }
        ),
    }
}
