use super::Puzzle;
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

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
    idx: usize,
    rx: Receiver<usize>,
    steps: Option<usize>,
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
        let mut channels = addrs
            .iter()
            .enumerate()
            .map(|(idx, addr)| {
                let (tx, rx) = sync_channel(1024);
                let addr = addr.clone();
                let directions = directions.clone();
                let nodes = nodes.clone();
                std::thread::spawn(move || spin(tx, addr, &directions, &nodes));
                Path {
                    idx,
                    rx,
                    steps: None,
                }
            })
            .collect_vec();

        let mut steps = 0usize;
        loop {
            for c in channels.iter_mut() {
                if c.steps.is_none() || c.steps.unwrap() < steps {
                    let s = c.rx.recv().unwrap();
                    // println!("{}\t|\t{}: {}  - {}", steps, c.idx, addr, s);
                    c.steps = Some(s);
                    steps = steps.max(s);
                }
            }
            if channels.iter().all(|c| c.steps == Some(steps)) {
                return Ok(steps.to_string());
            }
        }
    }
}

fn spin(
    tx: SyncSender<usize>,
    mut addr: String,
    directions: &[Direction],
    nodes: &HashMap<String, Node>,
) {
    let mut steps = 0usize;
    loop {
        for d in directions {
            match d {
                Direction::Left => addr = nodes[&addr].left.clone(),
                Direction::Right => addr = nodes[&addr].right.clone(),
            }
            steps += 1;
            if addr.ends_with("Z") {
                if tx.send(steps).is_err() {
                    return;
                }
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
