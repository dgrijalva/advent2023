use super::Puzzle;
use crate::{Direction, Grid, Pos};
use rayon::prelude::*;
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

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
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
        input.debug_print(|_pos, tile| format!("{:?}", tile));
        let result = summarize_ray(Ray::start(), &input);
        Ok(result.to_string())
    }

    fn part_two(&self, input: &str) -> super::PuzzleResult {
        let input: Grid<Tile> = input.parse().unwrap();
        let size = input.size();
        let top = (0..size.x).map(|x| Pos { x, y: 0 }).map(|pos| Ray {
            pos,
            facing: Direction::South,
        });
        let bottom = (0..size.x)
            .map(|x| Pos { x, y: size.y - 1 })
            .map(|pos| Ray {
                pos,
                facing: Direction::North,
            });
        let left = (0..size.y).map(|y| Pos { x: 0, y }).map(|pos| Ray {
            pos,
            facing: Direction::East,
        });
        let right = (0..size.y)
            .map(|y| Pos { x: size.x - 1, y })
            .map(|pos| Ray {
                pos,
                facing: Direction::West,
            });

        let result = top
            .chain(bottom)
            .chain(left)
            .chain(right)
            .par_bridge()
            .map(|ray| summarize_ray(ray, &input))
            .max()
            .unwrap();
        return Ok(result.to_string());
    }
}

/// Apply a ray to the grid and return the number of cells it energizes
/// (this is basically all of part 1)
fn summarize_ray(ray: Ray, grid: &Grid<Tile>) -> usize {
    let size = grid.size();
    let mut rays = vec![ray];
    let mut seen_rays = HashSet::new();
    let mut visited: Grid<bool> = Grid::new(size.x, size.y, false);

    // Walk each ray, one at a time
    while let Some(mut ray) = rays.pop() {
        log::debug!("Following: {ray:?}");
        let start = ray.pos;
        let mut loop_detection = HashSet::new();
        loop_detection.insert(ray);

        visited.set(&ray.pos, true);
        loop {
            let (done, new_ray) = ray.step(grid);
            if let Some(r) = new_ray {
                if !seen_rays.contains(&r) {
                    log::debug!("New ray {r:?}");
                    rays.push(r.clone());
                    seen_rays.insert(r);
                }
            }
            if done {
                break;
            } else {
                visited.set(&ray.pos, true);
                if loop_detection.contains(&ray) {
                    break;
                }
                loop_detection.insert(ray);
            }
        }
        print_grid(&visited, Some(start));
        log::debug!("");
    }

    print_grid(&visited, None);
    visited
        .rows()
        .map(|row| row.filter(|&&v| v).count())
        .sum::<usize>()
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
                log::debug!("[/] {:?} {:?}", self.pos, self.facing);
                match self.facing {
                    Direction::North => self.facing = Direction::East,
                    Direction::East => self.facing = Direction::North,
                    Direction::South => self.facing = Direction::West,
                    Direction::West => self.facing = Direction::South,
                };
                log::debug!("{:?}", self.facing);
                self.step_unchecked(grid, None)
            }
            Tile::NEMirror => {
                // "\"
                log::debug!("[\\] {:?} {:?}", self.pos, self.facing);
                match self.facing {
                    Direction::North => self.facing = Direction::West,
                    Direction::East => self.facing = Direction::South,
                    Direction::South => self.facing = Direction::East,
                    Direction::West => self.facing = Direction::North,
                };
                log::debug!("{:?}", self.facing);
                self.step_unchecked(grid, None)
            }
            Tile::HorizSplit => match self.facing {
                Direction::East | Direction::West => self.step_unchecked(grid, None),
                Direction::North | Direction::South => {
                    log::debug!("[-] {:?} {:?}", self.pos, self.facing);
                    let mut other = self.clone();
                    other.facing = Direction::East;
                    self.facing = Direction::West;
                    log::debug!("{:?}", self.facing);
                    self.step_unchecked(grid, Some(other))
                }
            },
            Tile::VertSplit => match self.facing {
                Direction::North | Direction::South => self.step_unchecked(grid, None),
                Direction::East | Direction::West => {
                    log::debug!("[|] {:?} {:?}", self.pos, self.facing);
                    let mut other = self.clone();
                    other.facing = Direction::South;
                    self.facing = Direction::North;
                    log::debug!("{:?}", self.facing);
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
        // log::debug!("{s}");
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
