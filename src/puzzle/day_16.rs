use itertools::Itertools;

use super::Puzzle;
use crate::{Direction, Grid, Pos};
use std::{collections::HashSet, str::FromStr};

pub struct Day16;

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    NWMirror,
    NEMirror,
    HorizSplit,
    VertSplit,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Ray {
    pos: Pos,
    facing: Direction,
}

impl Puzzle for Day16 {
    fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
        Box::new(Self)
    }

    fn part_one(&self, input: &str) -> super::PuzzleResult {
        let input: Grid<Tile> = input.parse().unwrap();
        input.debug_print(|pos, tile| format!("{:?}", tile));

        let size = input.size();
        let mut rays = vec![Ray::start()];
        let mut seen_rays = HashSet::new();
        let mut visited: Grid<bool> = Grid::new(size.x, size.y, false);

        // Walk each ray, one at a time
        while let Some(mut ray) = rays.pop() {
            println!("Following: {ray:?}");
            let start = ray.pos;
            let mut dbg_visited: Grid<bool> = Grid::new(size.x, size.y, false);

            visited.set(&ray.pos, true);
            dbg_visited.set(&ray.pos, true);
            loop {
                let (done, new_ray) = ray.step(&input);
                if let Some(r) = new_ray {
                    if !seen_rays.contains(&r) {
                        println!("New ray {r:?}");
                        rays.push(r.clone());
                        seen_rays.insert(r);
                    }
                }
                if done {
                    break;
                } else {
                    visited.set(&ray.pos, true);
                    dbg_visited.set(&ray.pos, true);
                }
            }
            print_grid(&visited, Some(start));
            println!("");
        }

        print_grid(&visited, None);
        let result = visited
            .rows()
            .map(|row| row.filter(|&&v| v).count())
            .sum::<usize>();
        Ok(result.to_string())
    }

    fn part_two(&self, _input: &str) -> super::PuzzleResult {
        todo!("implement part two")
    }
}

impl Ray {
    fn start() -> Self {
        Ray {
            pos: Pos::from((0, 0)),
            facing: Direction::East,
        }
    }

    /// Step forward. Returns true on done
    /// If there's a split, return one of them and follows the other
    fn step(&mut self, grid: &Grid<Tile>) -> (bool, Option<Ray>) {
        let value = *grid.value(&self.pos).unwrap();
        match value {
            Tile::Empty => self.step_unchecked(grid, None),
            Tile::NWMirror => {
                // "/"
                println!("[/] {:?} {:?}", self.pos, self.facing);
                match self.facing {
                    Direction::North => self.facing = Direction::East,
                    Direction::East => self.facing = Direction::North,
                    Direction::South => self.facing = Direction::West,
                    Direction::West => self.facing = Direction::South,
                };
                println!("{:?}", self.facing);
                self.step_unchecked(grid, None)
            }
            Tile::NEMirror => {
                // "\"
                println!("[\\] {:?} {:?}", self.pos, self.facing);
                match self.facing {
                    Direction::North => self.facing = Direction::West,
                    Direction::East => self.facing = Direction::South,
                    Direction::South => self.facing = Direction::East,
                    Direction::West => self.facing = Direction::North,
                };
                println!("{:?}", self.facing);
                self.step_unchecked(grid, None)
            }
            Tile::HorizSplit => match self.facing {
                Direction::East | Direction::West => self.step_unchecked(grid, None),
                Direction::North | Direction::South => {
                    println!("[-] {:?} {:?}", self.pos, self.facing);
                    let mut other = self.clone();
                    other.facing = Direction::East;
                    self.facing = Direction::West;
                    println!("{:?}", self.facing);
                    self.step_unchecked(grid, Some(other))
                }
            },
            Tile::VertSplit => match self.facing {
                Direction::North | Direction::South => self.step_unchecked(grid, None),
                Direction::East | Direction::West => {
                    println!("[|] {:?} {:?}", self.pos, self.facing);
                    let mut other = self.clone();
                    other.facing = Direction::South;
                    self.facing = Direction::North;
                    println!("{:?}", self.facing);
                    self.step_unchecked(grid, Some(other))
                }
            },
        }
    }

    /// Just walk in a direction without looking at the tile
    fn step_unchecked(&mut self, grid: &Grid<Tile>, other: Option<Ray>) -> (bool, Option<Ray>) {
        match grid.step(&self.pos, self.facing) {
            Some(p) => {
                self.pos = p;
                (false, other)
            }
            None => (true, other),
        }
    }
}

impl FromStr for Tile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("{s}");
        match s {
            "." => Ok(Self::Empty),
            "/" => Ok(Self::NWMirror),
            "\\" => Ok(Self::NEMirror),
            "-" => Ok(Self::HorizSplit),
            "|" => Ok(Self::VertSplit),
            other => Err(anyhow::anyhow!("Unexpected symbol: {other}")),
        }
    }
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Empty => ".",
            Self::NWMirror => "/",
            Self::NEMirror => "\\",
            Self::HorizSplit => "-",
            Self::VertSplit => "|",
        };
        f.write_str(s)
    }
}

fn print_grid(grid: &Grid<bool>, highlight: Option<Pos>) {
    grid.debug_print(|pos, c| {
        if Some(pos) == highlight {
            "*".to_string()
        } else if *c {
            "#".to_string()
        } else {
            ".".to_string()
        }
    });
}
