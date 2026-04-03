enum Command {
    Say(String),
    Quit,
}

fn handle_command(cmd: Command) -> String {
    match cmd {
        Command::Say(s) => s,
        Command::Quit => "Goodbye !".to_string(),
    }
}

fn main() {
    let bye = handle_command(Command::Quit);
    let command = handle_command(Command::Say("Hi Rustlers !".to_string()));
    println!("{}", command);
    println!("{}", bye);
}
