mod messages;
mod parser;

use messages::log_message::LogMessage;

fn main() {
    let lines = [
        "I 147 mice in the air, I’m afraid, but you might catch a bat",
        "E 2 148 #56k istereadeat lo d200ff] BOOTMEM",
    ];

    let info_message: Option<LogMessage> = parser::parse_line(lines.get(0).unwrap());
    println!("{:#?}", info_message.unwrap());

    let error_message: Option<LogMessage> = parser::parse_line(lines.get(1).unwrap());
    println!("{:#?}", error_message.unwrap());
}
