use clap::{Args, Subcommand};
use std::path::PathBuf;

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
    doc_path: Option<PathBuf>,
}

impl TrebuchetArgs {
    pub fn run(&self) -> String {
        match &self.command {
            Some(Commands::Calibrate(args)) => {
                let doc_path = args.doc_path.as_ref().unwrap();
                let result = calibrate(doc_path);
                format!("'calibrate' called with {:?}, result: {}", args, result)
            }
            None => String::from("Trebuchet!"),
        }
    }
}

fn calibrate(_file: &PathBuf) -> i32 {
    return 0;
}
