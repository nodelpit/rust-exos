// Struct contenant les états du programme
struct State {
    x: i32,
    y: i32,
    running: bool,
}

// Enum décrivant les événements (ce qui peut se passer dans le programme)
enum Action {
    MoveUp(i32),
    MoveDown(i32),
    MoveLeft(i32),
    MoveRight(i32),
    Quit,
}

impl State {
    fn apply(&mut self, action: Action) {
        match action {
            Action::MoveUp(u) => self.y += u,
            Action::MoveDown(e) => self.y -= e,
            Action::MoveLeft(l) => self.x -= l,
            Action::MoveRight(r) => self.x += r,
            Action::Quit => self.running = false,
        }
    }
}

fn main() {
    let mut state = State {
        x: 0,
        y: 0,
        running: true,
    };

    state.apply(Action::MoveUp(10));
    println!("( {},{} - running : {} )", state.x, state.y, state.running);

    state.apply(Action::MoveRight(5));
    println!("( {},{} - running : {} )", state.x, state.y, state.running);

    state.apply(Action::Quit);
    println!("( running : {} )", state.running);
}
