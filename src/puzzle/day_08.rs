use super::Puzzle;
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

pub struct Day08;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Node {
    id: String,
    left: String,
    right: String,
}

struct Path {
    #[allow(dead_code)] // for logging
    idx: usize,
    period: usize,
    steps: usize,
}

impl Puzzle for Day08 {
    fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
        Box::new(Self)
    }

    fn part_one(&self, input: &str) -> super::PuzzleResult {
        let (directions, nodes) = parse_input(input);

        let mut current_addr = "AAA".to_string();
        let mut steps = 0usize;
        loop {
            for d in &directions {
                println!("{}: {}", steps, current_addr);
                match d {
                    Direction::Left => current_addr = nodes[&current_addr].left.clone(),
                    Direction::Right => current_addr = nodes[&current_addr].right.clone(),
                }
                steps += 1;
                if current_addr == "ZZZ" {
                    return Ok(steps.to_string());
                }
            }
        }
    }

    fn part_two(&self, input: &str) -> super::PuzzleResult {
        let (directions, nodes) = parse_input(input);

        let addrs: Vec<_> = nodes
            .iter()
            .filter(|(id, _)| id.ends_with("A"))
            .map(|(id, _)| id.clone())
            .collect();
        println!("{:?} starting addrs end with A", addrs.len());

        // Spawn a thread for each starting address
        let mut paths = addrs
            .iter()
            .enumerate()
            .map(|(idx, addr)| {
                let period = compute_period(addr.clone(), &directions, &nodes);
                Path {
                    idx,
                    period,
                    steps: 0,
                }
            })
            .collect_vec();

        let mut steps = 2usize;
        loop {
            for c in paths.iter_mut() {
                if c.steps < steps {
                    let s = c.next();
                    // println!("{}\t|\t{} - {}", steps, c.idx, s);
                    steps = steps.max(s);
                }
            }
            if paths.iter().all(|c| c.steps == steps) {
                return Ok(steps.to_string());
            }
        }
    }
}

impl Path {
    fn next(&mut self) -> usize {
        self.steps = self.steps + self.period;
        self.steps
    }
}

fn compute_period(
    mut addr: String,
    directions: &[Direction],
    nodes: &HashMap<String, Node>,
) -> usize {
    let mut steps = 0usize;
    loop {
        for d in directions {
            match d {
                Direction::Left => addr = nodes[&addr].left.clone(),
                Direction::Right => addr = nodes[&addr].right.clone(),
            }
            steps += 1;
            if addr.ends_with("Z") {
                return steps;
            }
        }
    }
}

fn parse_input(input: &str) -> (Vec<Direction>, HashMap<String, Node>) {
    let mut lines = input.lines();

    let directions: Vec<Direction> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();

    let nodes: HashMap<String, Node> = lines
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .map(|n: Node| (n.id.clone(), n))
        .collect();

    (directions, nodes)
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(anyhow::anyhow!("invalid direction: {}", s)),
        }
    }
}

impl FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .replace('=', "")
            .replace('(', "")
            .replace(')', "")
            .replace(',', "");
        let mut parts = s.split(' ').filter(|s| !s.is_empty());

        let id = parts.next().unwrap().to_string();
        let left = parts.next().unwrap().to_string();
        let right = parts.next().unwrap().to_string();

        Ok(Self { id, left, right })
    }
}
