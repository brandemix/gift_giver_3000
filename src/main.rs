use clap::{Parser, Subcommand};
use historian::HistorianArgs;
use reactor::ReactorArgs;
use trebuchet::TrebuchetArgs;

mod historian;
mod reactor;
mod trebuchet;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct App {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Trebuchet(TrebuchetArgs),
    Historian(HistorianArgs),
    Reactor(ReactorArgs),
}

fn main() {
    let cli = App::parse();

    match &cli.command {
        Some(Commands::Trebuchet(args)) => {
            println!("'trebuchet' invoked: {}", args.run());
        }
        Some(Commands::Historian(args)) => {
            println!("'historian' invoked: {}", args.run())
        }
        Some(Commands::Reactor(args)) => {
            println!("'reactor' invoked: {}", args.run());
        }
        None => {
            println!("Default subcommand");
        }
    }
}
