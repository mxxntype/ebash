use std::io::{self, Write};

const PROMPT: &str = "$ ";
const KNOWN_COMMANDS: &[&str] = &[];

fn main() {
    let _ = color_eyre::install().inspect_err(|e| eprintln!("Could not install color-eyre: {e:?}"));

    let stdin = io::stdin();
    let mut commandline = String::new();

    loop {
        print!("{PROMPT}");
        io::stdout().flush().unwrap();

        match stdin.read_line(&mut commandline) {
            Ok(bytes) if bytes > 0 => {
                commandline = commandline.trim().into();

                if !KNOWN_COMMANDS.contains(&commandline.as_str()) {
                    println!("{commandline}: command not found");
                    io::stdout().flush().unwrap();
                }
            }
            _ => break,
        }

        commandline.clear();
    }
}
