use std::io::{self, Write};

const PROMPT: &str = "$ ";
const KNOWN_COMMANDS: &[&str] = &[];

fn main() {
    let _ = color_eyre::install().inspect_err(|e| eprintln!("Could not install color-eyre: {e:?}"));

    print!("{PROMPT}");
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    let mut commandline = String::new();
    stdin.read_line(&mut commandline).unwrap();
    let commandline = commandline.trim();

    if !KNOWN_COMMANDS.contains(&commandline) {
        println!("{commandline}: command not found");
        io::stdout().flush().unwrap();
    }
}
