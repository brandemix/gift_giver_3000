use clap::{Args, Subcommand};
use std::{io::BufReader, path::PathBuf};
use std::io::BufRead;
use std::collections::HashMap;

#[derive(Args)]
pub struct HistorianArgs {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Compile(CompileArgs),
}

#[derive(Args)]
struct CompileArgs {
    doc_path: Option<PathBuf>,
}

impl HistorianArgs {
    pub fn run(&self) -> String {
        match &self.command {
            Some(Commands::Compile(args)) => {
                let doc_path = args.doc_path.as_ref().unwrap();
                let result = compile(doc_path);
                format!("'compile' called. result: {:?}", result)
            }
            None => String::from("Historian!"),
        }
    }
}

pub fn compile(file: &PathBuf) -> (i64, i64) {
    let file = std::fs::File::open(file).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let mut locations: (Vec<i64>, Vec<i64>) = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split("   ").collect();
            println!("parts: {:?}", parts);
            (parts[0].parse::<i64>().unwrap(), parts[1].parse::<i64>().unwrap())
        }).collect();
        locations.0.sort();
        locations.1.sort();

    
    let mut similarity_score1 = 0;
    for i in 0..locations.0.len() {
        let diff: i64 = (locations.0[i] - locations.1[i]).abs();
        similarity_score1 += diff;
    }

    let mut frequencies: HashMap<i64, i64> = HashMap::new();
    for i in locations.1.iter() {
        frequencies.entry(*i).and_modify(|frequency| *frequency += 1).or_insert(1);
    }

    println!("frequences: {:?}", frequencies);

    let mut similarity_score2 = 0;
    for i in locations.0.iter() {
        let frequency = frequencies.get(i).or(Some(&0)).unwrap();
        similarity_score2 += i * frequency
    }

    return (similarity_score1, similarity_score2);
}