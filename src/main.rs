use std::io::{self, Write};

const PROMPT: &str = "$ ";

fn main() {
    let _ = color_eyre::install().inspect_err(|e| eprintln!("Could not install color-eyre: {e:?}"));

    let stdin = io::stdin();
    let mut commandline = String::new();

    loop {
        print!("{PROMPT}");
        io::stdout().flush().unwrap();

        match stdin.read_line(&mut commandline) {
            Ok(bytes) if bytes > 0 => {
                let commandline = commandline.trim();

                let words = commandline
                    .split_whitespace()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>();
                match words.first().unwrap_or(&String::new()).as_str() {
                    "exit" => std::process::exit(
                        words
                            .get(1)
                            .and_then(|w| w.parse::<i32>().ok())
                            .unwrap_or(0),
                    ),

                    "echo" => println!(
                        "{}",
                        words.into_iter().skip(1).collect::<Vec<_>>().join(" ")
                    ),

                    _ => println!("{commandline}: command not found"),
                }
                io::stdout().flush().unwrap();
            }
            _ => break,
        }

        commandline.clear();
    }
}
