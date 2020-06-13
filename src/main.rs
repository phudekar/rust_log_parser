mod messages;
mod parser;

fn main() {
    println!("Hello, world!");
    let lines = [
        "I 147 mice in the air, I’m afraid, but you might catch a bat",
        "E 2 148 #56k istereadeat lo d200ff] BOOTMEM",
    ];
    let message: Option<Box<dyn messages::log_message::LogMessage>> =
        parser::parse_line(lines.first().unwrap());
    println!("{:#?}", message.unwrap().message_type())
}
