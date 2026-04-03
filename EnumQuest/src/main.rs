enum Mode {
    Easy,
    Normal,
    Hard,
}

fn describe_mode(mode: Mode) -> &'static str {
    match mode {
        Mode::Easy => "Mode facile",
        Mode::Normal => "Mode normal",
        Mode::Hard => "Mode difficile",
    }
}
fn main() {
    let mode: Mode = Mode::Normal;
    let current_mode: &str = describe_mode(mode);
    println!("Vous avez choisi le mode {}", current_mode)
}
