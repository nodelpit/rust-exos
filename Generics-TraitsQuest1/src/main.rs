use std::mem;

struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Container<T> {
        Container { value }
    }

    fn get(&self) -> &T {
        &self.value
    }

    fn replace(&mut self, new_value: T) -> T {
        mem::replace(&mut self.value, new_value)
    }
}

fn main() {
    let mut container1 = Container::new(5);
    let mut container2 = Container::new(String::from("Hello"));

    println!("Avant replace: {}", container1.get());
    println!("Avant replace: {}", container2.get());

    let old1 = container1.replace(42);
    let old2 = container2.replace(String::from("Rust"));

    println!("==========================");

    println!("Ancienne valeur 1: {}", old1);
    println!("Ancienne valeur 2: {}", old2);

    println!("Nouvelle valeur 1: {}", container1.get());
    println!("Nouvelle valeur 2: {}", container2.get());
}
