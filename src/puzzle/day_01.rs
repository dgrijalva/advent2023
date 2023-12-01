use super::Puzzle;
use itertools::Itertools;

pub struct Day01;

impl Puzzle for Day01 {
    fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
        Box::new(Self)
    }

    fn part_one(&self, input: &str) -> super::PuzzleResult {
        Ok(input
            .split("\n")
            .filter(|line| !line.is_empty())
            .map(|line| {
                let numbers = line.chars().filter_map(|c| c.to_digit(10)).collect_vec();
                (numbers.first().unwrap() * 10) + numbers.last().unwrap()
            })
            .sum::<u32>()
            .to_string())
    }

    fn part_two(&self, _input: &str) -> super::PuzzleResult {
        todo!()
    }
}
