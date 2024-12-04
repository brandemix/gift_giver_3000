use clap::{Args, Parser, Subcommand};
use regex::Regex;
use std::io::Read;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Memory {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Execute(ExecuteArgs),
}

#[derive(Args)]
struct ExecuteArgs {
    doc_path: Option<PathBuf>,
}

impl Memory {
    pub fn run(&self) -> String {
        match &self.command {
            Some(Commands::Execute(args)) => {
                let doc_path = args.doc_path.as_ref().unwrap();
                let result = execute(doc_path);
                format!("'execute' called. result: {:?}", result)
            }
            None => String::from(""),
        }
    }
}

/* Runs a corrupted section of memory.
 * Supports
 *  - mul(x, y) instruction where x and y are 1 to 3 digit integers.
 *  - do() and don't() flow control
*/
pub fn execute(memory_file: &PathBuf) -> (i64, i64) {
    let mut file = std::fs::File::open(memory_file).expect("File wasn't found.");
    let mut memory = String::new();
    file.read_to_string(&mut memory).unwrap();

    let result1 = execute_section(&memory);
    let result2 = execute_complex(&memory);

    (result1, result2)
}

/* Runs a section of memory with do() and don't() flow control ENABLED*/
pub fn execute_complex(section: &str) -> i64 {
    section
        .split("don't()")
        .enumerate()
        .map(|(i, b)| {
            let pattern = if i == 0 {
                b
            } else {
                b.split_once("do()").map_or("", |(_, right)| right)
            };
            execute_section(pattern)
        })
        .sum()
}

/* Runs a section of memory with do() and don't() flow controll DISABLED */
fn execute_section(section: &str) -> i64 {
    let pattern = Regex::new(r"mul\((?<l>\d{1,3}),(?<r>\d{1,3})\)").unwrap();
    pattern
        .captures_iter(&section)
        .map(|nums| {
            let num1 = nums["l"].parse::<i64>().unwrap();
            let num2 = nums["r"].parse::<i64>().unwrap();
            num1 * num2
        })
        .sum()
}
