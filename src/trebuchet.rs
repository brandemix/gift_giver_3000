use clap::{Args, Subcommand};

#[derive(Args)]
pub struct TrebuchetArgs {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Calibrate(CalibrateArgs),
}

#[derive(Debug, Args)]
struct CalibrateArgs {
    doc_path: Option<String>,
}

impl TrebuchetArgs {
    pub fn run(&self) -> String {
        match &self.command {
            Some(Commands::Calibrate(args)) => format!("'calibrate' called with {:?}", args),
            None => String::from("Trebuchet!"),
        }
    }
}
