use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(_) => {
            println!("Unimplemented")
        }
        None => {
            println!("Default subcommand");
        }
    }
}
