enum Action {
    MoveUp(i32),
    MoveDown(i32),
    MoveLeft(i32),
    MoveRight(i32),
    Damage(i32),
    Heal(i32),
    Die,
}

struct Player {
    x: i32,
    y: i32,
    health: i32,
    alive: bool,
}

impl Player {
    fn apply(&mut self, action: Action) {
        if !self.alive && matches!(action, Action::MoveUp(_) | Action::MoveDown(_) | Action::MoveLeft(_) | Action::MoveRight(_) | Action::Heal(_)) { return; }
        match action {

            Action::MoveUp(u) => self.y += u,
            Action::MoveDown(d) => self.y -= d,
            Action::MoveLeft(l) => self.x -= l,
            Action::MoveRight(r) => self.x += r,

            Action::Damage(dmg) => {
                self.health -= dmg;
                if self.health <= 0 {
                    self.alive = false;
                }
            }
            Action::Heal(h) => {
                if self.alive == true {
                    self.health += h;
                }
            }

            Action::Die => {
                self.alive = false && self.health == 0;
            }
        }
    }

    fn status(&self) -> String {
        format!("Le joueur est a la position ( {},{} ). Il lui reste {} HP. Encore en vie ? {} !", self.x, self.y, self.health, self.alive)
    }
}

fn main() {
    let mut player = Player {
        x: 0,
        y: 0,
        health: 57,
        alive: true,
    };

    let move_right = Action::MoveRight(7);
    player.apply(move_right);
    println!("Le Player a bouger a la position ( {},{} )", player.x, player.y);

    let player_status = player.status();
    println!("Stats du joueur : {}", player_status);
}
