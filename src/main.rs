mod shell;

use shell::Shell;

fn main() {
    let _ = color_eyre::install().inspect_err(|e| eprintln!("Could not install color-eyre: {e:?}"));

    let mut shell = Shell::default();
    shell.run();
}
