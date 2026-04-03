struct Player {
        name: String,
        health: i32,
        alive: bool,
        x: i32,
        y: i32,
    }

    enum Action {
        MoveUp(i32),
        MoveDown(i32),
        MoveLeft(i32),
        MoveRight(i32),
        Damage(i32),
        Heal(i32),
        Die,
    }

    // Un bloc `impl` définit le comportement et la logique associée à une struct ou un enum
    impl Player {
        fn apply(&mut self, action: Action) {
            match action {
                Action::MoveUp(u) => self.y += u,
                Action::MoveDown(d) => self.y -= d,
                Action::MoveLeft(l) => self.x -= l,
                Action::MoveRight(r) => self.x += r,

                Action::Damage(n) => {
                    self.health -= n;
                        if self.health <= 0 {
                        self.alive = false;
                    }
                }
                Action::Heal(h) => {
                    if self.alive {
                        self.health += h;
                    }
                }
                Action::Die => self.alive = false,
            }
        }

        fn is_alive(&self) -> bool {
            self.alive
        }

        fn position(&self) -> (i32, i32) {
            (self.x, self.y)
        }

        fn status(&self) -> String {
            format!("{} est à ({},{}) avec {} HP. Toujours en vie ? : {}", self.name, self.x, self.y, self.health, self.alive)
        }
    }

fn main() {
    let mut player = Player {
        name: "RustY".to_string(),
        health: 58,
        alive: true,
        x: 10,
        y: 10,
    };

    let move_up = Action::MoveUp(5);
    player.apply(move_up);
    println!("{} est à la position ({},{}) avec {} HP.", player.name, player.x, player.y, player.health);

    let dmg = Action::Damage(8);
    player.apply(dmg);
    println!("Vous avez pris des dégats ! Votre santé : {}", player.health);

    if !player.is_alive() {
        println!("☠️ Game Over");
    }
}
