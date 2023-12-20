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
            // .into_par_iter()
            .into_iter()
            // .map(|s| (s.arrangements(), s))
            // .inspect(|v| {
            //     dbg!(v);
            // })
            // .map(|(c, _)| c)
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
                // println!("Valid {:?} {:?}", states, self.runs);
                return 1;
            } else {
                return 0;
            }
        };

        let mut data = states.to_vec();
        data[idx] = State::Operational;
        let left = self.possible_arrangements(&data);
        data[idx] = State::Damaged;
        let right = self.possible_arrangements(&data);
        left + right
    }

    fn complete(states: &[State]) -> bool {
        return !states.iter().any(|s| *s == State::Unknown);
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

    fn segments(&self) -> Vec<Vec<State>> {
        let mut segments = vec![];
        let mut current_segment = vec![];
        for state in self.states.iter().cloned() {
            match state {
                State::Damaged | State::Unknown => {
                    if current_segment.is_empty()
                        || !current_segment.iter().any(|s| *s == State::Operational)
                    {
                        current_segment.push(state);
                    } else {
                        segments.push(current_segment);
                        current_segment = vec![state];
                    }
                }
                State::Operational => {
                    if current_segment.is_empty()
                        || current_segment.iter().all(|s| *s == State::Operational)
                    {
                        current_segment.push(state);
                    } else {
                        segments.push(current_segment);
                        current_segment = vec![state];
                    }
                }
            }
        }
        if !current_segment.is_empty() {
            segments.push(current_segment);
        }

        segments
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
