struct Note {
    content: Box<String>,
}

impl Note {
    fn new(content: &str) -> Note {
        Note {
            content: Box::new(content.to_string())
        }
    }

    fn get_content(&self) -> &str {
        &self.content
    }

    fn add_text(&mut self, new_content: &str) {
        self.content.push_str(new_content)
    }
}

fn main() {
    let mut message = Note::new("Rust box");

    println!("{}", message.get_content());

    message.add_text(" here");

    println!("{}", message.get_content());
}
