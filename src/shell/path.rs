use super::Binary;
use std::io::{self, Error};
use std::process::{Command, Output};
use std::{env, fs};

pub fn path() -> Vec<Binary> {
    let paths = env::var_os("PATH").unwrap_or_default();
    env::split_paths(&paths)
        .flat_map(|path| fs::read_dir(path).and_then(Iterator::collect::<Result<Vec<_>, _>>))
        .flatten()
        .collect::<Vec<_>>()
}

pub(super) fn execute(command: &str, args: &[String]) -> io::Result<Output> {
    match path().iter().find(|f| f.file_name() == command) {
        Some(bin) => Command::new(bin.path()).args(args).output(),
        None => Err(Error::new(
            io::ErrorKind::NotFound,
            format!("{command}: command not found"),
        )),
    }
}
