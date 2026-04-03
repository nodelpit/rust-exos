struct Player {
    name: String,
    x: i32,
    y: i32,
    health: u32,
    alive: bool,
}

impl Player {
    fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
            x: 0,
            y: 0,
            health: 100,
            alive: true,
        }
    }

    // Méthodes utilisé pour bouger le joueur sur des positions x et y.

    fn move_up(&mut self, u: i32,) {
        self.y += u;
    }

    fn move_down(&mut self, d: i32,) {
        self.y -= d;
    }

    fn move_left(&mut self, l: i32,) {
        self.x -= l;
    }

    fn move_right(&mut self, r: i32,) {
        self.x += r;
    }

    //  Méthodes pour gérer la vie du joueur

    fn take_damage(&mut self, hit: u32) {
        if self.alive {
            self.health -= hit;
            if self.health <= 0 {
                self.alive = false;
            }
        }
    }

    fn heal(&mut self, h: u32) {
       if !self.alive {
        println!("Vous ne pouvez pas vous soignez !!!");
       } else {
        self.health += h;
       }
    }

    //  Méthodes pour voir les infos du joueur

    fn is_alive(&self) -> bool {
        self.alive
    }

    fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn status(&self) -> String {
        format!("{} est à ({},{}) avec {} HP - Toujours en vie ? : {}", self.name, self.x, self.y, self.health, self.alive)
    }

}

fn main() {
    let mut player = Player::new("Rusty");

    let player_status = player.status();
    println!("Voici les données de votre joueur : {}", player_status);

    player.move_left(10);
    let (x, y) = player.position();

    println!("{} s'est déplacé vers la gauche, coordonnées : ({},{})", player.name, x, y);

    let damage = player.take_damage(5);
    println!("{} a subit des dégats ! Il lui reste {}", player.name, player.health);
}
