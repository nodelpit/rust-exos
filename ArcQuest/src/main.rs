use std::sync::Arc;

fn main() {
    let s = Arc::new(String::from("mode=debug;version=1.0"));

    let s2 = Arc::clone(&s);
    let s3 = Arc::clone(&s);
    let s4 = Arc::clone(&s);

    println!("{}", s);

    println!("\n");

    println!("{}", s2);
    println!("{}", s3);
    println!("{}", s4);

    println!("Nombres d'owner: {}", Arc::strong_count(&s));
}
