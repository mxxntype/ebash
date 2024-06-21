use builtins::Builtins;
use std::fs::DirEntry;
use std::io::{self, Write};
use std::process::Output;

mod builtins;
mod external;
mod path;

type Binary = DirEntry;

pub struct Shell<'a> {
    stdin: io::Stdin,
    command: String,
    args: Vec<String>,
    builtins: Builtins<'a>,
    exiting: bool,
}

impl<'a> Default for Shell<'a> {
    fn default() -> Self {
        Self {
            stdin: io::stdin(),
            command: String::with_capacity(32),
            args: vec![],
            exiting: false,
            builtins: Builtins::default(),
        }
    }
}

impl<'a> Shell<'a> {
    const PROMPT: &'static str = "$ ";

    pub fn run(&mut self) {
        loop {
            Self::print_prompt();
            self.read_commandline();

            if !self.command.is_empty() && !self.execute_builtin() {
                self.execute_external();
            }

            self.command.clear();
            self.args.clear();

            if self.exiting {
                break;
            }
        }
    }

    fn execute_builtin(&self) -> bool {
        match self.builtins.get(self.command.as_str()) {
            Some(handler) => {
                handler(&self.args.iter().map(String::as_str).collect::<Vec<&str>>());
                true
            }
            _ => false,
        }
    }

    fn execute_external(&mut self) {
        match path::execute(&self.command, self.args.as_slice()) {
            Ok(Output { stdout, .. }) => {
                println!("{}", String::from_utf8_lossy(&stdout).trim());
                Self::flush_stdout();
            }

            Err(err) => eprintln!("{}", err.into_inner().unwrap_or("Unknown error".into())),
        }
    }

    fn flush_stdout() {
        let _ = io::stdout().flush();
    }

    fn print_prompt() {
        print!("{}", Self::PROMPT);
        Self::flush_stdout();
    }

    fn read_commandline(&mut self) {
        let mut buffer = String::new();
        match self.stdin.read_line(&mut buffer) {
            Ok(1..) => {
                let mut tokens = buffer
                    .split_whitespace()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>();
                if !tokens.is_empty() {
                    self.command = tokens.remove(0);
                    self.args = tokens;
                }
            }

            // EOF caught (User hit Ctrl+D).
            Ok(0) => self.exiting = true,

            // Stdin is fucked somehow.
            Err(err) => eprintln!("Error reading from stdin: {err:?}"),
        };
    }
}
