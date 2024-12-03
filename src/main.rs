mod historian;
mod reactor;
mod trebuchet;

use clap::{Parser, Subcommand};
use historian::Historian;
use reactor::Reactor;
use trebuchet::Trebuchet;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct App {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Trebuchet(Trebuchet),
    Historian(Historian),
    Reactor(Reactor),
}

fn main() {
    let cli = App::parse();

    match &cli.command {
        Some(Commands::Trebuchet(trebuchet)) => {
            println!("'trebuchet' invoked: {}", trebuchet.run());
        }
        Some(Commands::Historian(historian)) => {
            println!("'historian' invoked: {}", historian.run());
        }
        Some(Commands::Reactor(reactor)) => {
            println!("'reactor' invoked: {}", reactor.run());
        }
        None => {
            println!("Default subcommand");
        }
    }
}
