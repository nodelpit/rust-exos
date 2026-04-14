use std::collections::HashMap;

fn main() {
    let mut data = HashMap::new();

    data.insert("mode", "debug");
    data.insert("version", "1.0");
    data.insert("env", "dev");

    match data.get("mode") {
        Some(m) => println!("{}", m),
        None => println!("Cette clé n'existe pas"),
    }
}
