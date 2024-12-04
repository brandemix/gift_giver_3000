use clap::{Args, Parser, Subcommand};
use std::io::BufRead;
use std::{io::BufReader, path::PathBuf};

#[derive(Parser)]
pub struct Puzzles {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Xmas(ExecuteArgs),
}

#[derive(Args)]
struct ExecuteArgs {
    doc_path: Option<PathBuf>,
}

impl Puzzles {
    pub fn run(&self) -> String {
        match &self.command {
            Some(Commands::Xmas(args)) => {
                let doc_path = args.doc_path.as_ref().unwrap();
                let result = xmas(doc_path);
                format!("XMAS word search: {:?} ocurrences", result)
            }
            None => String::from("Play XMAS word search!"),
        }
    }
}

pub fn xmas(input: &PathBuf) -> (i64, i64) {
    let file = std::fs::File::open(input).expect("File wasn't found.");
    let reader = BufReader::new(file);
    let puzzle: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect();

    if puzzle.len() == 0 {
        return (0, 0);
    }

    let x = puzzle[0].len();
    let y = puzzle.len();

    let mut xmas_found = 0;
    let mut x_mas_found = 0;

    for i in 0..y {
        for j in 0..x {
            if puzzle[i][j] == 'A'
                && (j as i32 - 1 >= 0 && i as i32 - 1 >= 0 && j + 1 < y && i + 1 < x)
                && ((puzzle[i - 1][j - 1] == 'M' && puzzle[i + 1][j + 1] == 'S')
                    || (puzzle[i - 1][j - 1] == 'S' && puzzle[i + 1][j + 1] == 'M'))
                && ((puzzle[i - 1][j + 1] == 'M' && puzzle[i + 1][j - 1] == 'S')
                    || (puzzle[i - 1][j + 1] == 'S' && puzzle[i + 1][j - 1] == 'M'))
            {
                x_mas_found += 1;
            }

            if puzzle[i][j] == 'X' {
                // Check left
                if j as i32 - 3 >= 0
                    && (puzzle[i][j - 1] == 'M'
                        && puzzle[i][j - 2] == 'A'
                        && puzzle[i][j - 3] == 'S')
                {
                    xmas_found += 1;
                }

                // Check left-up diagonal
                if j as i32 - 3 >= 0
                    && i as i32 - 3 >= 0
                    && (puzzle[i - 1][j - 1] == 'M'
                        && puzzle[i - 2][j - 2] == 'A'
                        && puzzle[i - 3][j - 3] == 'S')
                {
                    xmas_found += 1;
                }

                // Check up
                if i as i32 - 3 >= 0
                    && (puzzle[i - 1][j] == 'M'
                        && puzzle[i - 2][j] == 'A'
                        && puzzle[i - 3][j] == 'S')
                {
                    xmas_found += 1;
                }

                // Check right-up diagonal
                if i as i32 - 3 >= 0
                    && j + 3 < x
                    && (puzzle[i - 1][j + 1] == 'M'
                        && puzzle[i - 2][j + 2] == 'A'
                        && puzzle[i - 3][j + 3] == 'S')
                {
                    xmas_found += 1;
                }

                // Check right
                if j + 3 < x
                    && (puzzle[i][j + 1] == 'M'
                        && puzzle[i][j + 2] == 'A'
                        && puzzle[i][j + 3] == 'S')
                {
                    xmas_found += 1;
                }

                // Check right-down diagonal
                if i + 3 < y
                    && j + 3 < x
                    && (puzzle[i + 1][j + 1] == 'M'
                        && puzzle[i + 2][j + 2] == 'A'
                        && puzzle[i + 3][j + 3] == 'S')
                {
                    xmas_found += 1;
                }

                // Check down
                if i + 3 < y
                    && (puzzle[i + 1][j] == 'M'
                        && puzzle[i + 2][j] == 'A'
                        && puzzle[i + 3][j] == 'S')
                {
                    xmas_found += 1;
                }

                // Check left-down diagonal
                if i + 3 < y
                    && j as i32 - 3 >= 0
                    && (puzzle[i + 1][j - 1] == 'M'
                        && puzzle[i + 2][j - 2] == 'A'
                        && puzzle[i + 3][j - 3] == 'S')
                {
                    xmas_found += 1;
                }
            }
        }
    }

    return (xmas_found, x_mas_found);
}
