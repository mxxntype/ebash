use std::{
    collections::HashMap,
    io::{self, Write},
};

const PROMPT: &str = "$ ";

type Command = &'static str;
type Handler<'a> = Box<dyn Fn(&[&str]) + 'a>;

fn main() {
    let _ = color_eyre::install().inspect_err(|e| eprintln!("Could not install color-eyre: {e:?}"));

    let stdin = io::stdin();
    let mut commandline = String::new();
    let mut buitins: HashMap<Command, Handler> = HashMap::new();

    buitins.insert("exit", command_exit());
    buitins.insert("echo", command_echo());
    buitins.insert("type", Box::new(|_| unreachable!()));
    buitins.insert(
        "type",
        command_type(buitins.keys().copied().collect::<Vec<_>>()),
    );

    loop {
        print!("{PROMPT}");
        io::stdout().flush().unwrap();

        match stdin.read_line(&mut commandline) {
            Ok(bytes) if bytes > 0 => {
                let commandline = commandline.split_whitespace().collect::<Vec<&str>>();

                let command = commandline.first().copied().unwrap_or("");
                let args = commandline.into_iter().skip(1).collect::<Vec<&str>>();

                match buitins.get(command) {
                    Some(handler) => handler(&args),
                    None => println!("{command}: command not found"),
                }

                io::stdout().flush().unwrap();
            }
            _ => break,
        }

        commandline.clear();
    }
}

fn command_exit() -> Handler<'static> {
    Box::new(|args: &[&str]| {
        std::process::exit(
            args.first()
                .and_then(|a| a.parse::<i32>().ok())
                .unwrap_or(0),
        );
    })
}

fn command_echo() -> Handler<'static> {
    Box::new(|args: &[&str]| {
        println!("{}", args.join(" "));
    })
}

#[allow(clippy::match_bool)]
fn command_type(buitins: Vec<&str>) -> Handler {
    Box::new(move |args: &[&str]| {
        let command = args.first().unwrap_or(&"");
        match buitins.contains(command) {
            true => println!("{command} is a shell builtin"),
            false => println!("{command}: command not found"),
        }
    })
}
