#[derive(Debug, Clone)]
struct Message {
    content: String,
}

impl Message {
    fn new(message: &str) -> Message {
        Message { content: message.to_string() }
    }
}

fn main() {
    let message1 = Message::new("rust");
    let message2 = message1.clone();

    println!("{:?}", message1);
    println!("{:?}", message2);
}
