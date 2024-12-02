use clap::{Args, Subcommand};
use std::collections::HashMap;
use std::io::BufRead;
use std::{io::BufReader, path::PathBuf};

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

    /* Read in location ids from the two lists */
    let mut locations: (Vec<i64>, Vec<i64>) = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split_whitespace().collect();
            println!("parts: {:?}", parts);
            (
                parts[0].parse::<i64>().unwrap(),
                parts[1].parse::<i64>().unwrap(),
            )
        })
        .collect();

    /* Sort our lists */
    locations.0.sort();
    locations.1.sort();

    /* Compile our naive similarity score (part 1) */
    let similarity_score1 = locations
        .0
        .iter()
        .zip(locations.1.iter())
        .map(|(l1, l2)| (l1 - l2).abs())
        .sum();

    /* Compile frequency of each location id in list 2 */
    let frequencies: HashMap<i64, i64> =
        locations.1.into_iter().fold(HashMap::new(), |mut acc, l| {
            acc.entry(l).and_modify(|f| *f += 1).or_insert(1);
            return acc;
        });

    /* Compile sophisticated similarity score (part 2) using compiled frequencies */
    let similarity_score2 = locations
        .0
        .iter()
        .map(|l| l * frequencies.get(l).or(Some(&0)).unwrap())
        .sum();

    return (similarity_score1, similarity_score2);
}
