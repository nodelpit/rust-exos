struct Metadata {
    author: String,
    priority: i32,
}

struct Message {
    content: String,
    metadata: Box<Metadata>,
}

impl Metadata {
    fn new(author: &str, priority: i32) -> Metadata {
        Metadata {
            author: author.to_string(),
            priority: priority,
        }
    }
}

impl Message {
    fn new(content: &str, author: &str, priority: i32) -> Message {
        Message {
            content: content.to_string(),
            metadata: Box::new(Metadata::new(author, priority)),
        }
    }

    fn get_author(&self) -> &str {
        &self.metadata.author
    }

    fn get_priority(&self) -> &i32 {
        &self.metadata.priority
    }

    fn increase_priority(&mut self) {
        self.metadata.priority += 1;
    }

    fn display(&self) {
        println!(
"Message: {}
Author: {}
Priority: {}",
            self.content, self.metadata.author, self.metadata.priority,
        )
    }
}

fn main() {    
    let mut message = Message::new("go learn rust !!", "Norusty", 9);

    message.increase_priority();
    message.display();
}
