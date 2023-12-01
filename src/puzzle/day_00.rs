//! This is a template for a puzzle solution.  Copy this file to a new file.
//! Files in this folder are auto-discovered at build time.

use super::Puzzle;

pub struct Day00;

impl Puzzle for Day00 {
    fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
        Box::new(Self)
    }

    fn part_one(&self, _input: &str) -> super::PuzzleResult {
        todo!("implement part one")
    }

    fn part_two(&self, _input: &str) -> super::PuzzleResult {
        todo!("implement part two")
    }
}
