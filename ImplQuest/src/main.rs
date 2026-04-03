struct Player {
    name: String,
    health: i32,
    x: i32,
    y: i32,
}

impl Player {
    fn is_alive(&self) -> bool {
        self.health > 0
    }

    fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn status(&self) -> String {
        format!(
            "Bonjour {}, vous êtes ici : '{},{}'. Il vous reste {} HP",
            self.name, self.x, self.y, self.health
        )
    }
}

fn main() {
    let player = Player {
        name: "Rusty".to_string(),
        health: 85,
        x: 12,
        y: 16,
    };

    let is_alive = player.is_alive();
    let position = player.position();
    let status = player.status();

    println!("{} est encore en vie ? : {} !", player.name, is_alive);
    println!("Votre position : ( {},{} )", position.0, position.1);
    println!("{}", status);
}
