use super::Puzzle;
use itertools::Itertools;
use std::collections::VecDeque;

pub struct Day09;

impl Puzzle for Day09 {
    fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
        Box::new(Self)
    }

    fn part_one(&self, input: &str) -> super::PuzzleResult {
        let input = parse_input(input);
        let result = input
            .into_iter()
            .map(|line| compute_next(line))
            .sum::<i64>();
        Ok(result.to_string())
    }

    fn part_two(&self, _input: &str) -> super::PuzzleResult {
        todo!("implement part two")
    }
}

fn compute_next(mut line: Vec<i64>) -> i64 {
    let mut stack = VecDeque::new();
    stack.push_front(line.clone());
    loop {
        let next_line = differences(&line);
        if next_line.iter().all(|&v| v == 0) {
            break;
        }
        stack.push_front(next_line.clone());
        line = next_line;
    }

    let mut diff = 0i64;
    for line in stack.drain(..) {
        diff = line.last().unwrap() + diff;
        // println!("{:?} -> {}", line, diff);
    }

    diff
}

fn differences(line: &[i64]) -> Vec<i64> {
    let mut result = vec![];
    let mut prev = line[0];
    for i in line[1..].iter() {
        result.push(i - prev);
        prev = *i;
    }
    result
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| line.split(' ').map(|n| n.parse().unwrap()).collect_vec())
        .collect_vec()
}
