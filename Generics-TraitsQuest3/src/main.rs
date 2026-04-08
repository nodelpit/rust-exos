struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Pair<T> {
        Pair { first, second }
    }

    fn get_first(&self) -> &T {
        &self.first
    }

    fn get_second(&self) -> &T {
        &self.second
    }
}

impl<T: PartialOrd> Pair<T> {
    fn largest(&self) -> &T {
        if &self.first > &self.second { &self.first } else { &self.second }
    }
}

fn main() {
    let numbers = Pair::new(10, 20);
    println!("Largest: {}", numbers.largest());

    let floats = Pair::new(3.22, 2.81);
    println!("Largest: {}", floats.largest());
}
