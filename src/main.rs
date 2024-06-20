use std::io::{self, Write};
use std::{collections::HashMap, fs::DirEntry};
use std::{env, fs, process, rc::Rc};

const PROMPT: &str = "$ ";

type Command = &'static str;
type Handler<'a> = Box<dyn Fn(&[&str]) + 'a>;

fn main() {
    let _ = color_eyre::install().inspect_err(|e| eprintln!("Could not install color-eyre: {e:?}"));

    let stdin = io::stdin();
    let mut commandline = String::new();
    let mut buitins: HashMap<Command, Handler> = HashMap::new();

    let path = Rc::new(
        env::split_paths(&env::var("PATH").unwrap_or_default())
            .flat_map(|path| fs::read_dir(path).and_then(Iterator::collect::<Result<Vec<_>, _>>))
            .flatten()
            .collect::<Vec<_>>(),
    );

    buitins.insert("exit", command_exit());
    buitins.insert("echo", command_echo());
    buitins.insert("type", Box::new(|_| unreachable!()));
    buitins.insert(
        "type",
        command_type(
            buitins.keys().copied().collect::<Vec<_>>(),
            Rc::clone(&path),
        ),
    );

    loop {
        print!("{PROMPT}");
        io::stdout().flush().unwrap();

        match stdin.read_line(&mut commandline) {
            Ok(bytes) if bytes > 0 => {
                let commandline = commandline.split_whitespace().collect::<Vec<&str>>();
                let command = commandline.first().copied().unwrap_or("");
                let args = commandline.into_iter().skip(1).collect::<Vec<&str>>();
                let external = path.iter().find(|file| file.file_name() == command);
                let builtin = buitins.get(command);

                match (builtin, external) {
                    (Some(handler), _) => handler(&args),
                    (None, Some(bin)) => {
                        let stdout = process::Command::new(bin.path())
                            .args(args)
                            .output()
                            .unwrap()
                            .stdout;
                        println!("{}", String::from_utf8(stdout).unwrap().trim_end());
                    }
                    (None, None) => println!("{command}: command not found"),
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
        process::exit(
            args.first()
                .and_then(|a| a.parse::<i32>().ok())
                .unwrap_or(0),
        );
    })
}

fn command_echo() -> Handler<'static> {
    Box::new(|args: &[&str]| println!("{}", args.join(" ")))
}

fn command_type(buitins: Vec<&str>, path: Rc<Vec<DirEntry>>) -> Handler {
    Box::new(move |args: &[&str]| {
        let command = args.first().unwrap_or(&"");
        let builtin = buitins.contains(command);
        let external = path.iter().find(|f| f.file_name() == *command);
        match (builtin, external) {
            (true, _) => println!("{command} is a shell builtin"),
            (false, Some(f)) => println!("{command} is {}", f.path().to_string_lossy()),
            (false, None) => println!("{command}: not found"),
        }
    })
}
