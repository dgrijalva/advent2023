use super::Puzzle;
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

impl Puzzle for Day08 {
    fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
        Box::new(Self)
    }

    fn part_one(&self, input: &str) -> super::PuzzleResult {
        let (directions, nodes) = parse_input(input);
        println!("directions: {:?}", directions);
        println!("nodes: {:?}", nodes);

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
                if steps % 1000 == 0 {
                    println!("{}: {}", steps, current_addr);
                }
                if current_addr == "ZZZ" {
                    return Ok(steps.to_string());
                }
            }
        }
    }

    fn part_two(&self, _input: &str) -> super::PuzzleResult {
        todo!("implement part two")
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
