use super::Puzzle;
use std::collections::HashSet;

pub struct Day04;

struct Card {
    winning: HashSet<usize>,
    values: Vec<usize>,
}

impl Puzzle for Day04 {
    fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
        Box::new(Self)
    }

    fn part_one(&self, input: &str) -> super::PuzzleResult {
        let cards = Self::parse(input);
        let score = cards
            .into_iter()
            .map(|card| {
                card.values
                    .iter()
                    .filter(|v| card.winning.contains(v))
                    .count()
            })
            .filter(|&c| c > 0)
            .map(|c| 2usize.pow(c as u32 - 1))
            .sum::<usize>();
        Ok(score.to_string())
    }

    fn part_two(&self, _input: &str) -> super::PuzzleResult {
        todo!("implement part two")
    }
}

impl Day04 {
    fn parse(input: &str) -> Vec<Card> {
        input
            .lines()
            .map(|line| {
                let mut parts = line[9..].split('|');
                let winning = parts
                    .next()
                    .unwrap()
                    .split(' ')
                    .filter(|v| !v.is_empty())
                    .map(|v| v.parse().unwrap())
                    .collect();
                let values = parts
                    .next()
                    .unwrap()
                    .split(' ')
                    .filter(|v| !v.is_empty())
                    .map(|v| v.parse().unwrap())
                    .collect();
                Card { winning, values }
            })
            .collect()
    }
}
