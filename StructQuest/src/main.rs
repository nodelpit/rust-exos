struct User {
    name: String,
    age: u32,
    active: bool,
}

impl User {
    fn new(name: &str) -> User {
        User {
            name: name.to_string(),
            age: 19,
            active: true,
        }
    }

    fn birthday(&mut self) {
        if self.active { self.age += 1 }
    }

    fn deactivate(&mut self) {
        self.active = false
    }

    fn is_active(&self) -> bool {
        self.active
    }
}


fn main() {
    let mut new_user = User::new("Rusty");

    println!("{} ({} ans) - actif: {}", new_user.name, new_user.age, new_user.is_active());

    new_user.birthday();

    println!("{} ({} ans) - actif: {}", new_user.name, new_user.age, new_user.is_active());

    new_user.deactivate();

    println!("{} ({} ans) - actif: {}", new_user.name, new_user.age, new_user.is_active());
}
