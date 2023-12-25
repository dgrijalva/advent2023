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
        let mut grid = parse_input(input);
        // for _ in 0..1000000000 {
        for _ in 0..1000000 {
            grid.tilt(Direction::North);
            grid.tilt(Direction::West);
            grid.tilt(Direction::South);
            grid.tilt(Direction::East);
        }
        Ok(grid.total_load().to_string())
    }
}

impl Grid {
    /// Shift in a direction until the pieces stop moving
    fn tilt(&mut self, direction: Direction) {
        loop {
            let move_count = self.shift(direction);
            if move_count == 0 {
                break;
            }
        }
    }

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

    fn swap(&mut self, pos: Pos, direction: Direction) -> bool {
        // println!("swap {pos:?} {direction:?}");
        let Some(swap_pos) = self.swap_pos(pos, direction) else {
            return false;
        };
        // println!("swap_pos {swap_pos:?}");
        let item = self.0[pos.y][pos.x];
        let swap = self.0[swap_pos.y][swap_pos.x];
        if item == Position::Sphere && swap == Position::Empty {
            self.0[pos.y][pos.x] = swap;
            self.0[swap_pos.y][swap_pos.x] = item;
            return true;
        }
        false
    }

    /// Shift all movable objects one unit in the specified direction
    /// Returns the number of objects that moved
    fn shift(&mut self, direction: Direction) -> usize {
        let size = self.size();
        let mut move_count = 0usize;

        // let (x_iter, y_iter) = match direction {
        //     Direction::North => ((0..size.x).collect_vec(), (0..size.y).collect_vec()),
        //     Direction::East => ((0..size.x).rev().collect_vec(), (0..size.y).collect_vec()),
        //     Direction::South => ((0..size.x).collect_vec(), (0..size.y).rev().collect_vec()),
        //     Direction::West => ((0..size.x).collect_vec(), (0..size.y).collect_vec()),
        // };

        match direction {
            Direction::North => {
                for y in 1..size.y {
                    for x in 0..size.x {
                        // println!("{x} {y}");
                        if self.swap((x, y).into(), direction) {
                            move_count += 1;
                        }
                    }
                }
            }
            Direction::South => {
                for y in (0..(size.y - 1)).rev() {
                    for x in 0..size.x {
                        if self.swap((x, y).into(), direction) {
                            move_count += 1;
                        }
                    }
                }
            }
            Direction::West => {
                for x in 1..size.x {
                    for y in 0..size.y {
                        if self.swap((x, y).into(), direction) {
                            move_count += 1;
                        }
                    }
                }
            }
            Direction::East => {
                for x in (0..(size.x - 1)).rev() {
                    for y in 0..size.y {
                        if self.swap((x, y).into(), direction) {
                            move_count += 1;
                        }
                    }
                }
            }
        }

        move_count
    }

    fn swap_pos(&self, pos: Pos, direction: Direction) -> Option<Pos> {
        match direction {
            Direction::North => {
                if pos.y == 0 {
                    return None;
                }
                Some(Pos {
                    x: pos.x,
                    y: pos.y - 1,
                })
            }
            Direction::South => {
                let new_y = pos.y + 1;
                if new_y >= self.size().y {
                    return None;
                }
                Some(Pos { x: pos.x, y: new_y })
            }
            Direction::West => {
                if pos.x == 0 {
                    return None;
                }
                Some(Pos {
                    x: pos.x - 1,
                    y: pos.y,
                })
            }
            Direction::East => {
                let new_x = pos.x + 1;
                if new_x >= self.size().x {
                    return None;
                }
                Some(Pos { x: new_x, y: pos.y })
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
