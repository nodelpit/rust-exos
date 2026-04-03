struct Player {
    name: String,
    health: i32,
    x: i32,
    y: i32,
}

impl Player {
    fn move_up(&mut self, n: i32) {
        self.y += n;
    }

    fn move_down(&mut self, n: i32) {
        self.y -= n;
    }

    fn move_left(&mut self, n: i32) {
        self.x -= n;
    }

    fn move_right(&mut self, n: i32) {
        self.x += n;
    }

    fn take_damage(&mut self, dmg: i32) {
        self.health -= dmg;
        if self.health < 0 {
            self.health = 0;
        }
    }

    fn heal(&mut self, amount: i32) {
        self.health += amount
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }

    fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

fn main() {
    let mut player = Player {
        name: "RustY".to_string(),
        health: 67,
        x: 23,
        y: 27,
    };

    player.move_up(2);
    println!("1ere position du joueur : ( {},{} )", player.x, player.y);

    player.move_right(5);
    println!("2eme position du jouer : {:?}", player.position());

    player.take_damage(30);
    println!(
        "Santé après avoir pris des dommages : {}, En vie ? {}",
        player.health,
        player.is_alive()
    );

    player.heal(20);
    println!(
        "Santé après avoir pris des soins : {}, En vie ? {}",
        player.health,
        player.is_alive()
    );
}
