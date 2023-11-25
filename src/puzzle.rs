use clap::Parser;

macros::import_solutions!();

use crate::RootOpt;

pub type PuzzleResult = Result<String, anyhow::Error>;

pub trait Puzzle {
    fn new(ops: &RootOpt) -> Self;

    fn part_one(input: &str) -> PuzzleResult {
        todo!("Implement part one");
    }

    fn part_two(input: &str) -> PuzzleResult {
        todo!("Implement part two");
    }
}

#[derive(Clone, Debug, Parser)]
pub struct PuzzleCommand {
    /// Submit the result and update the data files
    #[arg(long)]
    commit: bool,
}

impl PuzzleCommand {
    fn run(&self, opt: &RootOpt) -> Result<(), anyhow::Error> {
        todo!();
    }
}
