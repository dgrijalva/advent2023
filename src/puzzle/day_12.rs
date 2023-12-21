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
        let result = data.into_iter().map(|s| s.arrangements()).sum::<usize>();
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
        self.possible_arrangements(&self.states)
    }

    fn possible_arrangements(&self, states: &[State]) -> usize {
        // find index of first unknown state
        let Some(idx) = states
            .iter()
            .enumerate()
            .find(|(_, &s)| s == State::Unknown)
            .map(|(i, _)| i)
        else {
            if self.valid(states) {
                // double check
                // if self.runs != Self::run_lengths(states) {
                //     panic!(
                //         "Invalid state: {:?} | {:?} != {:?}",
                //         states,
                //         Self::run_lengths(states),
                //         self.runs
                //     );
                // }

                // println!("Valid {:?} {:?}", states, self.runs);
                return 1;
            } else {
                return 0;
            }
        };

        // recursive depth first search
        let mut sum = 0usize;
        let mut data = states.to_vec();
        for s in [State::Damaged, State::Operational] {
            data[idx] = s;
            // short circuit if already invalid
            if self.valid(&data) {
                sum += self.possible_arrangements(&data);
            }
        }
        sum
    }

    fn valid(&self, states: &[State]) -> bool {
        let mut runs = self.runs.iter();
        let mut in_run = None;
        let mut run_len = 0usize;
        for state in states {
            match state {
                State::Unknown => return true, // TODO
                State::Operational => {
                    // end run, if there is one
                    if let Some(&run) = in_run {
                        // make sure run was expected len
                        if run_len != run {
                            return false;
                        }
                    }
                    in_run = None;
                    run_len = 0;
                }
                State::Damaged => {
                    if in_run.is_none() {
                        in_run = runs.next();
                        // no more runs
                        if in_run.is_none() {
                            return false;
                        }
                    }
                    run_len += 1;
                }
            }
        }
        // catch the last run if it goes to the end
        if let Some(&run) = in_run {
            // make sure run was expected len
            if run_len != run {
                return false;
            }
        }
        if runs.next().is_some() {
            return false;
        }
        true
    }

    /// An alternate approach to validation, used to double-check my work but no longer needed
    #[allow(unused)]
    fn run_lengths(states: &[State]) -> Vec<usize> {
        let mut lengths = vec![];
        let mut run_len = 0usize;
        for s in states {
            match s {
                State::Unknown => unreachable!("only use this on complete states"),
                State::Damaged => {
                    run_len += 1;
                }
                State::Operational => {
                    if run_len > 0 {
                        lengths.push(run_len);
                    }
                    run_len = 0;
                }
            }
        }
        if run_len > 0 {
            lengths.push(run_len);
        }
        lengths
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
