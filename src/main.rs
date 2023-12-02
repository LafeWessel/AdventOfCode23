mod puzzles;

use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use puzzles::{
    puzzle::Puzzle,
    puzzle_01::{Puzzle011, Puzzle012},
};

#[derive(Clone, ValueEnum)]
enum Puzzles {
    Puzzle011,
    Puzzle012,
}

impl Puzzles {
    fn puzzle(&self) -> Box<dyn Puzzle> {
        match self {
            Puzzles::Puzzle011 => Box::new(Puzzle011 {}),
            Puzzles::Puzzle012 => Box::new(Puzzle012 {}),
        }
    }

    fn puzzle_num(&self) -> u32 {
        match self {
            Puzzles::Puzzle011 | Puzzles::Puzzle012 => 1,
        }
    }
}

#[derive(Parser)]
#[command(author, version, about)]
struct Interface {
    /// Puzzle to solve
    #[arg(short, long, value_name = "PUZZLE")]
    puzzle: Puzzles,
    /// Path to input file
    #[arg(short, long, value_name = "INPUT")]
    input_file: Option<String>,
}

fn main() {
    let inf = Interface::parse();

    let path = if let Some(ref path) = inf.input_file {
        path.clone()
    } else {
        format!("puzzle_input/puzzle_{:0>2}.txt", inf.puzzle.puzzle_num())
    };

    if !PathBuf::from(&path).exists() {
        panic!("Input path {path:?} does not exist");
    }

    let res = inf.puzzle.puzzle().solve(&path);
    println!("{res}");
}
