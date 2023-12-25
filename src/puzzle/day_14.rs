use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use itertools::Itertools;

use super::Puzzle;
use crate::Pos;

pub struct Day14;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Grid(Vec<Vec<Position>>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Position {
    Empty,
    Cube,
    Sphere,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Puzzle for Day14 {
    fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
        Box::new(Self)
    }

    fn part_one(&self, input: &str) -> super::PuzzleResult {
        let mut grid = parse_input(input);
        grid.tilt(Direction::North);
        Ok(grid.total_load().to_string())
    }

    fn part_two(&self, input: &str) -> super::PuzzleResult {
        let input = parse_input(input);
        let mut grid = &input;
        let mut memo: HashMap<Grid, Grid> = Default::default();
        for _ in 0..1000000000 {
            // for _ in 0..1000000 {
            if let Some(next) = memo.get(grid) {
                grid = next;
                continue;
            }

            let prev = grid.clone();
            let mut next = grid.clone();
            next.tilt(Direction::North);
            next.tilt(Direction::West);
            next.tilt(Direction::South);
            next.tilt(Direction::East);
            memo.insert(prev.clone(), next);
            grid = memo.get(&prev).unwrap();
        }
        Ok(grid.total_load().to_string())
    }
}

impl Grid {
    fn total_load(&self) -> usize {
        let size = self.size();
        let mut load = 0usize;
        for (y, row) in self.0.iter().enumerate() {
            let mut count = 0usize;
            for (_x, cell) in row.iter().enumerate() {
                if *cell == Position::Sphere {
                    count += 1;
                }
            }
            load += count * (size.y - y);
        }

        load
    }

    fn size(&self) -> Pos {
        Pos::size_of(&self.0)
    }

    #[inline]
    fn swap(&mut self, pos: Pos, open: &mut VecDeque<Pos>) {
        let x = pos.x;
        let y = pos.y;
        let data = self.0[y][x];
        match data {
            Position::Empty => open.push_back(pos),
            Position::Cube => open.clear(),
            Position::Sphere => {
                if let Some(op) = open.pop_front() {
                    self.0[y][x] = Position::Empty;
                    self.0[op.y][op.x] = Position::Sphere;
                    open.push_back(pos);
                }
            }
        }
    }

    /// Shift all movable objects one unit in the specified direction
    /// Returns the number of objects that moved
    fn tilt(&mut self, direction: Direction) {
        let size = self.size();
        let mut open_pos: VecDeque<Pos> = VecDeque::with_capacity(32);

        match direction {
            Direction::North => {
                for x in 0..size.x {
                    open_pos.clear();
                    for y in 0..size.y {
                        self.swap((x, y).into(), &mut open_pos);
                    }
                }
            }
            Direction::South => {
                for x in 0..size.x {
                    open_pos.clear();
                    for y in (0..size.y).rev() {
                        self.swap((x, y).into(), &mut open_pos);
                    }
                }
            }
            Direction::West => {
                for y in 0..size.y {
                    open_pos.clear();
                    for x in 0..size.x {
                        self.swap((x, y).into(), &mut open_pos);
                    }
                }
            }
            Direction::East => {
                for y in 0..size.y {
                    open_pos.clear();
                    for x in (0..size.x).rev() {
                        self.swap((x, y).into(), &mut open_pos);
                    }
                }
            }
        }
    }
}

fn parse_input(input: &str) -> Grid {
    let data = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect_vec()
        })
        .collect_vec();

    Grid(data)
}

impl FromStr for Position {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::Empty),
            "#" => Ok(Self::Cube),
            "O" => Ok(Self::Sphere),
            other => Err(anyhow::anyhow!("Unexpected symbol: {other}")),
        }
    }
}
