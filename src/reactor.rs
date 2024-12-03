use clap::{Args, Parser, Subcommand};
use std::io::BufRead;
use std::{io::BufReader, path::PathBuf};

/**
 * codename: Red-Nosed reactor
 */

#[derive(Parser)]
pub struct Reactor {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Verify(VerifyArgs),
}

#[derive(Args)]
struct VerifyArgs {
    doc_path: Option<PathBuf>,
}

impl Reactor {
    pub fn run(&self) -> String {
        match &self.command {
            Some(Commands::Verify(args)) => {
                let doc_path = args.doc_path.as_ref().unwrap();
                let result = verify(doc_path);
                format!("'verify' called. result: {:?}", result)
            }
            None => String::from("Red-Nosed reactor: 267°C"),
        }
    }
}

pub fn verify(report_file: &PathBuf) -> (i64, i64) {
    let file = std::fs::File::open(report_file).expect("File wasn't found.");
    let reports = BufReader::new(file);

    let safe_count = reports
        .lines()
        .map(|report| {
            let report = report.unwrap();
            let report_levels: Vec<i64> = report
                .split_whitespace()
                .map(|level| level.parse::<i64>().unwrap())
                .collect();
            let result_without_dampening = verify_report(report_levels.iter().collect());
            let result_with_dampening = verify_report_dampen(report_levels);
            (
                result_without_dampening as i64,
                result_with_dampening as i64,
            )
        })
        .fold((0, 0), |acc, (a, b)| (acc.0 + a, acc.1 + b));

    return safe_count;
}

pub fn verify_report_dampen(levels: Vec<i64>) -> bool {
    /* Search sublists by removing random element */
    (0..levels.len()).any(|i| verify_report(levels[0..i].iter().chain(&levels[i + 1..]).collect()))
}

pub fn verify_report(levels: Vec<&i64>) -> bool {
    /* - The levels are either all increasing or all decreasing.
     * - Any two adjacent levels differ by at least one and at most three. */
    let mut ord: Result<i64, ()> = Ok(0);
    for i in 1..levels.len() {
        match ord {
            Ok(a) => {
                if a >= 0 && (1..=3).contains(&(levels[i] - levels[i - 1])) {
                    ord = Ok(1);
                } else if a <= 0 && (1..=3).contains(&(levels[i - 1] - levels[i])) {
                    ord = Ok(-1);
                } else {
                    ord = Err(())
                }
            }
            Err(_) => ord = Err(()),
        }
    }

    match ord {
        Ok(_) => true,
        Err(_) => false,
    }
}
