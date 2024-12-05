use clap::{Args, Parser, Subcommand};
use std::collections::HashSet;
use std::io::BufRead;
use std::{io::BufReader, path::PathBuf};

#[derive(Parser)]
pub struct Printer {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Check(CheckArgs),
}

#[derive(Args)]
struct CheckArgs {
    doc_path: Option<PathBuf>,
}

impl Printer {
    pub fn run(&self) -> String {
        match &self.command {
            Some(Commands::Check(args)) => {
                let doc_path = args.doc_path.as_ref().unwrap();
                let result = check(doc_path);
                format!("Checking ordering rules... {}", result)
            }
            None => String::from("Printer says: Hello!"),
        }
    }
}

pub fn check(input: &PathBuf) -> i64 {
    let file = std::fs::File::open(input).expect("File wasn't found.");

    let mut reader = BufReader::new(file);

    let mut rules: HashSet<String> = HashSet::new();
    let mut buf = String::new();
    loop {
        let len = reader.read_line(&mut buf).unwrap();
        if len - 1 == 0 {
            break;
        }

        rules.insert(buf.trim().to_string());
        buf.clear();
    }

    println!("{:?}", rules);
    reader
        .lines()
        .map(|line| {
            let mut middle = 0;
            let mut valid = true;
            let updates: Vec<i64> = line
                .unwrap()
                .split(",")
                .map(|c| c.parse::<i64>().unwrap())
                .collect();
            if updates.len() < 2 {
                return 0;
            }

            let mut i = 0;
            let mut j = 1;
            loop {
                if !valid || i == updates.len() {
                    break;
                } else if j == updates.len() {
                    i += 1;
                    j = i + 1;
                    continue;
                }

                if !rules.contains(&format!("{}|{}", updates[i], updates[j])) {
                    valid = false;
                }

                j += 1;
            }

            if valid {
                middle = updates[updates.len() / 2];
            }

            return middle;
        })
        .sum()
}
