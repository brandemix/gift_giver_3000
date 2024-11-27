use clap::{Parser, Subcommand};
use trebuchet::TrebuchetArgs;

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
}

fn main() {
    let cli = App::parse();

    match &cli.command {
        Some(Commands::Trebuchet(args)) => {
            println!("'trebuchet' invoked: {}", args.run());
        }
        None => {
            println!("Default subcommand");
        }
    }
}
