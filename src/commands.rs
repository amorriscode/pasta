use clap::Command;

pub fn cli() -> Command {
    Command::new("pasta")
        .about("A lightweight clipboard manager for macOS")
        .subcommand(Command::new("start").about("Start pasteboard listener"))
}
