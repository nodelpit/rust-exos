enum Movement {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

// x et y sont les positions actuelles !
fn apply_movement(x: i32, y: i32, movement: Movement) -> (i32, i32) {
    match movement {
        Movement::Up(u) => (x, y + u as i32),
        Movement::Down(d) => (x, y - d as i32),
        Movement::Left(l) => (x - l as i32, y),
        Movement::Right(r) => (x + r as i32, y),
    }
}

fn main() {
    let x = 0;
    let y = 0;
    let movement_up = Movement::Up(10);

    let start = apply_movement(x, y, movement_up);
    println!("1er mouvement(vers le haut) {:?}", start);

    let movement_left = Movement::Left(5);

    let sec_mov = apply_movement(x, y, movement_left);
    println!("2eme mouvement (vers la gauche) {:?}", sec_mov);
}
