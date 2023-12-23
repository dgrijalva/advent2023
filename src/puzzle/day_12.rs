use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use super::Puzzle;
use std::{collections::HashMap, str::FromStr};

pub struct Day12;

#[derive(Debug, Clone)]
struct Sequence {
    states: Vec<State>,
    runs: Vec<usize>,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
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
            .into_iter()
            .map(|s| s.arrangements())
            .inspect(|v| println!("{v}"))
            .sum::<usize>();
        Ok(result.to_string())
    }

    fn part_two(&self, input: &str) -> super::PuzzleResult {
        let data = parse_input(input)
            .into_iter()
            .map(|s| s.unfold())
            .collect_vec();

        let result = data
            .into_par_iter()
            .map(|s| dbg!(s.arrangements()))
            .sum::<usize>();
        Ok(result.to_string())
    }
}

impl Sequence {
    fn unfold(&self) -> Self {
        let mut states = vec![];
        let mut runs = vec![];
        for i in 0..5 {
            states.extend(self.states.iter().copied());
            if i < 4 {
                states.push(State::Unknown);
            }
            runs.extend(self.runs.iter().copied());
        }

        Self { states, runs }
    }

    fn arrangements(&self) -> usize {
        // println!("start {:?} {:?}", &self.states, &self.runs);
        let mut memo = Default::default();
        Self::possible_arrangements(&mut memo, &self.states, &self.runs)
    }

    fn possible_arrangements<'a, 'b>(
        memo: &'b mut HashMap<(&'a [State], &'a [usize]), usize>,
        states: &'a [State],
        runs: &'a [usize],
    ) -> usize {
        if let Some(v) = memo.get(&(states, runs)) {
            return *v;
        }

        // if there are no more runs, there can't be any more damaged nodes
        if runs.is_empty() {
            if states.contains(&State::Damaged) {
                return 0;
            } else {
                return 1;
            }
        }

        // if states is empty, there can't be any more unconsumed runs
        if states.is_empty() {
            if runs.is_empty() {
                return 1;
            } else {
                return 0;
            }
        }

        // recursive depth first search
        let mut sum = 0usize;
        // Try the next state as . by just stepping forward
        if [State::Unknown, State::Operational].contains(&states[0]) {
            sum += Self::possible_arrangements(memo, &states[1..], runs);
        }
        // Try the next state as # by trying to consume the expected run length
        if [State::Unknown, State::Damaged].contains(&states[0]) {
            let next = runs[0];
            if states.len() >= next // There are enough states left
                && !states[..next].contains(&State::Operational)
            {
                // the run isn't too long
                if states.len() == next {
                    sum += Self::possible_arrangements(memo, &states[(next)..], &runs[1..]);
                } else if states[next] != State::Damaged {
                    sum += Self::possible_arrangements(memo, &states[(next + 1)..], &runs[1..]);
                }
            }
        }

        memo.insert((states, runs), sum);
        // println!("{states:?} {runs:?} {sum}");
        sum
    }
}

fn parse_input(input: &str) -> Vec<Sequence> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let parts = line.split(' ').collect_vec();
            Sequence {
                states: parts[0]
                    .chars()
                    .map(|c| c.to_string().parse().unwrap())
                    .collect_vec(),
                runs: parts[1]
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
