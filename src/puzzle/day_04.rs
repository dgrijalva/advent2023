use super::Puzzle;
use std::collections::{HashMap, HashSet};

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
            .map(|card| card.winning_count())
            .filter(|&c| c > 0)
            .map(|c| 2usize.pow(c as u32 - 1))
            .sum::<usize>();
        Ok(score.to_string())
    }

    fn part_two(&self, input: &str) -> super::PuzzleResult {
        let cards = Self::parse(input);
        let scores: Vec<(usize, usize)> = cards
            .iter()
            .map(|card| card.winning_count())
            .enumerate()
            .collect();
        let mut card_count = HashMap::new();
        for (idx, score) in &scores {
            // insert original card
            let count = inc(&mut card_count, *idx, 1);
            // insert won cards
            for i in (*idx + 1)..scores.len().min(*score + idx + 1) {
                inc(&mut card_count, i, count);
            }
        }

        let result = card_count
            .iter()
            .filter(|(idx, _)| **idx < cards.len())
            .map(|(_, v)| v)
            .sum::<usize>();
        Ok(result.to_string())
    }
}

impl Card {
    fn winning_count(&self) -> usize {
        self.values
            .iter()
            .filter(|v| self.winning.contains(v))
            .count()
    }
}

fn inc(data: &mut HashMap<usize, usize>, idx: usize, c: usize) -> usize {
    let count = data.entry(idx).or_insert(0);
    *count += c;
    *count
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
