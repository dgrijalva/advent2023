use std::str::FromStr;

use itertools::Itertools;

use super::Puzzle;
use crate::Pos;

pub struct Day14;

struct Grid(Vec<Vec<Position>>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Position {
    Empty,
    Cube,
    Sphere,
}

impl Puzzle for Day14 {
    fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
        Box::new(Self)
    }

    fn part_one(&self, input: &str) -> super::PuzzleResult {
        let mut grid = parse_input(input);
        loop {
            let move_count = grid.shift_n();
            if move_count == 0 {
                break;
            }
        }
        Ok(grid.total_load().to_string())
    }

    fn part_two(&self, _input: &str) -> super::PuzzleResult {
        todo!("implement part two")
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

    /// Shift all movable objects one unit north
    /// Returns the number of objects that moved
    fn shift_n(&mut self) -> usize {
        let size = self.size();
        let mut move_count = 0usize;
        for y in 1..size.y {
            for x in 0..size.x {
                let item = self.0[y][x];
                let above = self.0[y - 1][x];
                if item == Position::Sphere && above == Position::Empty {
                    self.0[y][x] = above;
                    self.0[y - 1][x] = item;
                    move_count += 1;
                }
            }
        }

        move_count
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
