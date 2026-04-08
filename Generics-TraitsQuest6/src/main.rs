#[derive(Debug)]
struct Comparator<T> {
    left: T,
    right: T,
}

impl<T> Comparator<T> {
    fn new(left: T, right: T) -> Comparator<T> {
        Comparator { left, right }
    }
}

trait Score {
    fn score(&self) -> i32;
}

impl<T: Score> Comparator<T> {
    fn best(&self) -> &T {
        if &self.left.score() > &self.right.score() { &self.left } else { &self.right }
    }
}

impl Score for i32 {
    fn score(&self) -> i32 {
        *self
    }
}

impl Score for String {
    fn score(&self) -> i32 {
        let longueur = self.len() as i32;
        longueur
    }
}    

fn main() {
    let numbers = Comparator::new(10, 17);
    println!("The best: {}", numbers.best());

    let character = Comparator::new("rust".to_string(), "generic".to_string());
    println!("The best: {}", character.best());
}