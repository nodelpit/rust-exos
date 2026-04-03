struct Position {
    x: i32,
    y: i32,
}

enum Action {
    Move(Position),
    Stop,
}

fn execute_action(current: Position, action: Action) -> Position {
    match action {
        Action::Move(p) => p,
        Action::Stop => current,
    }
}

fn main() {
    let current_position = Position { x: 0, y: 5 };

    let action1 = Action::Move(Position { x: 10, y: 15 });
    let current_position = execute_action(current_position, action1);

    println!(
        "Voici la position actuelle : ( {},{} )",
        current_position.x, current_position.y
    );

    let action2 = Action::Move(Position { x: 12, y: 17 });
    let current_position = execute_action(current_position, action2);

    println!(
        "Nouvelle position : ( {},{} )",
        current_position.x, current_position.y
    );

    let action3 = Action::Stop;
    let stop = execute_action(current_position, action3);

    println!("STOP ! Dernière position : ( {}, {} )", stop.x, stop.y);
}
