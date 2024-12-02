use clap::{Parser, Subcommand};
use historian::HistorianArgs;
use trebuchet::TrebuchetArgs;

mod trebuchet;
mod historian;

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
        None => {
            println!("Default subcommand");
        }
    }
}
