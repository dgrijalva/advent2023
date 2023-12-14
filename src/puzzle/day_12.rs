use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use super::Puzzle;
use std::str::FromStr;

pub struct Day12;

#[derive(Debug, Clone)]
struct Sequence {
    states: Vec<State>,
    runs: Vec<usize>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum State {
    Damaged,
    Operational,
    Unknown,
}

impl Puzzle for Day12 {
    fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
        Box::new(Self)
    }

    fn part_one(&self, input: &str) -> super::PuzzleResult {
        let data = parse_input(input);

        let result = data
            .into_par_iter()
            .map(|s| s.arrangements())
            .sum::<usize>();
        Ok(result.to_string())
    }

    fn part_two(&self, _input: &str) -> super::PuzzleResult {
        todo!("implement part two")
    }
}

impl Sequence {
    fn arrangements(&self) -> usize {
        println!("{:?}", self);
        0
    }
}

fn parse_input(input: &str) -> Vec<Sequence> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let mut parts = line.split(' ');
            Sequence {
                states: parts
                    .next()
                    .unwrap()
                    .chars()
                    .map(|c| c.to_string().parse().unwrap())
                    .collect_vec(),
                runs: parts
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|c| c.parse().unwrap())
                    .collect_vec(),
            }
        })
        .collect_vec()
}

impl FromStr for State {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Self::Damaged),
            "." => Ok(Self::Operational),
            "?" => Ok(Self::Unknown),
            _ => anyhow::bail!("invalid state: {}", s),
        }
    }
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Damaged => write!(f, "D"),
            Self::Operational => write!(f, "O"),
            Self::Unknown => write!(f, "?"),
        }
    }
}
