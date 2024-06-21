use std::{collections::HashMap, ops::Deref, process};

type Command = &'static str;
type Handler<'a> = Box<dyn Fn(&[&str]) + 'a>;

pub(super) struct Builtins<'a> {
    buitins: HashMap<Command, Handler<'a>>,
}

impl<'a> Deref for Builtins<'a> {
    type Target = HashMap<Command, Handler<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.buitins
    }
}

impl<'a> Default for Builtins<'a> {
    fn default() -> Self {
        let mut buitins: HashMap<Command, Handler> = HashMap::new();

        buitins.insert("exit", command_exit());
        buitins.insert("echo", command_echo());
        buitins.insert("type", Box::new(|_| unreachable!()));
        buitins.insert(
            "type",
            command_type(buitins.keys().copied().collect::<Vec<_>>()),
        );

        Self { buitins }
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

fn command_type(buitins: Vec<&str>) -> Handler {
    Box::new(move |args: &[&str]| {
        let path = super::path::path();
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
