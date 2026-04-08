use std::mem::swap;

#[derive(Debug)]
struct DualContainer<T, U> {
    first: T,
    second: U,
}

impl<T, U> DualContainer<T, U> {
    fn new(first: T, second: U) -> DualContainer<T, U> {
        DualContainer { first, second }
    }

    fn get_first(&self) -> &T{
        &self.first
    }

    fn get_second(&self) -> &U{
        &self.second
    }
}

impl<T> DualContainer<T, T> {
    fn swap(&mut self) {
        swap(&mut self.first, &mut self.second);
    }
}

fn main() {
    let first_and_second = DualContainer::new(13, 12.1);
    println!("Nos deux types: {:?}", first_and_second);

    println!("||=====================||");

    let get_first = first_and_second.get_first();
    println!("Le type first: {:?}", get_first);

    let get_second = first_and_second.get_second();
    println!("Le type second: {:?}", get_second);

    println!("||=====================||");

    let mut same_types = DualContainer::new("It's", "Okk");
    println!("Avant swap: {:?}", same_types);

    same_types.swap();

    println!("Après swap: {:?}", same_types);
}
