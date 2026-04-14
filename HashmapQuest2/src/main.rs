use std::collections::HashMap;

struct WordAnalytics {
    data: HashMap<String, u32>,
}

impl WordAnalytics {
    fn new() -> WordAnalytics {
        WordAnalytics {
            data: HashMap::new()
        }
    }

    fn process_text(&mut self, text: &str) {
        for word in text.split_whitespace() {
            *self.data.entry(word.to_string()).or_insert(0) += 1;
        }
    }

    fn display(&self) {
        for (k, v) in &self.data {
            println!("{} -> {}", k, v);
        }
    }
}

fn main() {
    let mut data = WordAnalytics::new();

    data.process_text("rust fr fr Fr Axum rust loco Loco");
    data.display();
}
