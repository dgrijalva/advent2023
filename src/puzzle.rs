use clap::Parser;
use macros::get_solution;

macros::import_solutions!();

use crate::RootOpt;

pub type PuzzleResult = Result<String, anyhow::Error>;

pub trait Puzzle {
    fn new(ops: &RootOpt) -> Box<dyn Puzzle>
    where
        Self: Sized;

    fn part_one(&self, _input: &str) -> PuzzleResult {
        todo!("Implement part one");
    }

    fn part_two(&self, _input: &str) -> PuzzleResult {
        todo!("Implement part two");
    }
}

#[derive(Clone, Debug, Parser, Default)]
pub struct PuzzleCommand {
    /// Submit the result and update the data files
    #[arg(long)]
    commit: bool,
}

impl PuzzleCommand {
    pub fn run(&self, opt: &RootOpt) -> Result<(), anyhow::Error> {
        let client = crate::client::Client::new(opt)?;
        let day = get_solution!(opt);
        let solution = match opt.part {
            1 => day.part_one(&client.get_input()?)?,
            2 => day.part_two(&client.get_input()?)?,
            _ => todo!("Implement part three"),
        };

        println!("Solution: {}", solution);
        Ok(())
    }
}
