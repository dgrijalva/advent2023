use itertools::Itertools;
use std::str::FromStr;

use super::Puzzle;
use crate::Pos;

pub struct Day13;

#[derive(Clone, Debug)]
struct Grid(Vec<Vec<Location>>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Location {
    Rock,
    Ash,
}

impl Puzzle for Day13 {
    fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
        Box::new(Self)
    }

    fn part_one(&self, input: &str) -> super::PuzzleResult {
        let input = parse_input(input);
        let result = input
            .iter()
            .map(|grid| {
                // +1 because we count the left side of the match
                let mut val = 0usize;
                if let Some(hi) = grid.horiz_reflection() {
                    val += (hi + 1) * 100;
                }
                if let Some(vi) = grid.vert_reflection() {
                    val += vi + 1;
                }
                val
            })
            .inspect(|v| println!("{v}"))
            .sum::<usize>();

        Ok(result.to_string())
    }

    fn part_two(&self, _input: &str) -> super::PuzzleResult {
        todo!("implement part two")
    }
}

impl Grid {
    fn row(&self, idx: usize) -> Vec<Location> {
        self.0[idx].clone()
    }

    fn col(&self, idx: usize) -> Vec<Location> {
        self.0.iter().map(|row| row[idx]).collect_vec()
    }

    fn size(&self) -> Pos {
        Pos {
            y: self.0.len(),
            x: self.0[0].len(),
        }
    }

    /// Returns the index of the top line of a reflection
    fn horiz_reflection(&self) -> Option<usize> {
        let candidate = self.find_reflection(Self::row, self.size().y)?;
        self.validate_reflection(Self::row, self.size().y, candidate)
        // Some(candidate)
    }

    fn vert_reflection(&self) -> Option<usize> {
        let candidate = self.find_reflection(Self::col, self.size().x)?;
        self.validate_reflection(Self::col, self.size().x, candidate)
        // Some(candidate)
    }

    fn find_reflection(
        &self,
        lookup: impl Fn(&Grid, usize) -> Vec<Location>,
        len: usize,
    ) -> Option<usize> {
        for i in 0..(len - 1) {
            if lookup(self, i) == lookup(self, i + 1) {
                return Some(i);
            }
        }
        None
    }

    fn validate_reflection(
        &self,
        lookup: impl Fn(&Grid, usize) -> Vec<Location>,
        len: usize,
        idx: usize,
    ) -> Option<usize> {
        if idx == 0 {
            return Some(idx);
        }

        let mut left = idx - 1;
        let mut right = idx + 2;

        while right < len {
            if lookup(self, left) != lookup(self, right) {
                return None;
            }

            if left > 0 {
                left -= 1;
                right += 1;
            } else {
                break;
            }
        }

        Some(idx)
    }
}

fn parse_input(input: &str) -> Vec<Grid> {
    input
        .split("\n\n")
        .map(|g| {
            Grid(
                g.lines()
                    .map(|l| {
                        l.chars()
                            .map(|c| c.to_string().parse().unwrap())
                            .collect_vec()
                    })
                    .collect_vec(),
            )
        })
        .collect_vec()
}

impl FromStr for Location {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::Ash),
            "#" => Ok(Self::Rock),
            other => Err(anyhow::anyhow!("Unexpected symbol: {other}")),
        }
    }
}
