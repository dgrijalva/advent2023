use itertools::Itertools;

use super::Puzzle;
use std::collections::HashSet;
use std::str::FromStr;

pub struct Day10;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Piece {
    Ground,
    Start,
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord(usize, usize);

impl Puzzle for Day10 {
    fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
        Box::new(Self)
    }

    fn part_one(&self, input: &str) -> super::PuzzleResult {
        let grid = parse_input(input);
        let start = find_start(&grid);
        println!("Start: {start:?}");
        let path = find_loop(start, &grid);
        let result = (path.len() / 2) + (path.len() & 1);
        Ok(result.to_string())
    }

    fn part_two(&self, input: &str) -> super::PuzzleResult {
        let grid = parse_input(input);
        let start = find_start(&grid);
        let mut path = find_loop(start, &grid)
            .into_iter()
            .map(|(c, _)| c)
            .collect_vec();
        let bounds = (grid[0].len(), grid.len());

        let mut enclosed = find_enclosed(&path, bounds);
        if enclosed.iter().any(|c| c.0 == 0 || c.1 == 0) {
            // we need to walk clockwise around the path for this to work.
            // i don't know how to determine that, so we'll just use the version
            // which doesn't select cells on the border
            path[1..].reverse();
            enclosed = find_enclosed(&path, bounds);
        }

        print_grid(&grid, &path, &enclosed.iter().copied().collect_vec());

        let result = enclosed.len();
        Ok(result.to_string())
    }
}

fn find_enclosed(path: &[Coord], bounds: (usize, usize)) -> HashSet<Coord> {
    let mut enclosed = HashSet::new();
    let mut prev = path[0];
    for (i, &current) in path[1..].iter().enumerate() {
        let heading = Direction::heading(prev, current);
        if let Some(right) = current.step(heading.turn_right()) {
            // println!("{:?} -> {:?} {:?}", prev, current, heading);
            if !path.contains(&right) {
                // enclosed.insert(right);
                flood_fill(right, &mut enclosed, &path, bounds)
            }
        }
        // We need to check both sides of a corner before moving on
        if let Some(next) = path.get(i + 2) {
            let heading = Direction::heading(current, *next);
            if let Some(right) = current.step(heading.turn_right()) {
                // println!("{:?} -> {:?} {:?}", prev, current, heading);
                if !path.contains(&right) {
                    // enclosed.insert(right);
                    flood_fill(right, &mut enclosed, &path, bounds)
                }
            }
        }
        prev = current;
    }

    enclosed
}

fn find_loop(start: Coord, grid: &[Vec<Piece>]) -> Vec<(Coord, Piece)> {
    // Try talking in each direction until we find a loop
    for dir in [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ] {
        if let Some(path) = walk(start, dir, grid) {
            return path;
        }
    }
    unreachable!("no loop found");
}

fn walk(start: Coord, mut dir: Direction, grid: &[Vec<Piece>]) -> Option<Vec<(Coord, Piece)>> {
    let mut visited = HashSet::new();

    let mut piece = start.lookup(grid)?;
    let mut path = vec![(start, piece)];

    // Reset visit history
    visited.clear();
    visited.insert(start);

    // Start by walking one unit in dir
    let mut current = start.step(dir)?;
    visited.insert(current);
    loop {
        piece = current.lookup(grid)?;
        if piece == Piece::Start {
            return Some(path);
        }
        path.push((current, piece));
        (current, dir) = current.follow(piece, dir)?;
    }
}

fn find_start(grid: &[Vec<Piece>]) -> Coord {
    grid.iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find(|(_, p)| **p == Piece::Start)
                .map(|(x, _)| Coord(x, y))
        })
        .unwrap()
}

fn parse_input(input: &str) -> Vec<Vec<Piece>> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn flood_fill(coord: Coord, data: &mut HashSet<Coord>, path: &[Coord], bounds: (usize, usize)) {
    type D = Direction;
    let mut next = vec![coord];
    while let Some(v) = next.pop() {
        data.insert(v);
        for d in [D::North, D::East, D::West, D::South] {
            let Some(c) = v.step(d) else {
                continue;
            };
            if c.0 >= bounds.0 || c.1 >= bounds.1 {
                continue;
            }
            if data.contains(&c) {
                continue;
            }
            if path.contains(&c) {
                continue;
            }
            next.push(c);
        }
    }
}

impl Coord {
    /// Get piece located at coord, if valid
    fn lookup(&self, grid: &[Vec<Piece>]) -> Option<Piece> {
        grid.get(self.1).and_then(|row| row.get(self.0)).cloned()
    }

    /// Walk in a direction
    fn step(&self, direction: Direction) -> Option<Coord> {
        Some(match direction {
            Direction::North => {
                if self.1 == 0 {
                    return None;
                }
                Coord(self.0, self.1 - 1)
            }
            Direction::South => Coord(self.0, self.1 + 1),
            Direction::West => {
                if self.0 == 0 {
                    return None;
                }
                Coord(self.0 - 1, self.1)
            }
            Direction::East => Coord(self.0 + 1, self.1),
        })
    }

    /// Follow a pipe. Returns next coord and output direction
    fn follow(&self, piece: Piece, direction: Direction) -> Option<(Coord, Direction)> {
        type D = Direction;
        let d = match (piece, direction) {
            (Piece::Ground, _) => return None,
            (Piece::Horizontal, D::East) => D::East,
            (Piece::Horizontal, D::West) => D::West,
            (Piece::Vertical, D::North) => D::North,
            (Piece::Vertical, D::South) => D::South,
            // L
            (Piece::NorthEast, D::South) => D::East,
            (Piece::NorthEast, D::West) => D::North,
            // J
            (Piece::NorthWest, D::South) => D::West,
            (Piece::NorthWest, D::East) => D::North,
            // 7
            (Piece::SouthWest, D::North) => D::West,
            (Piece::SouthWest, D::East) => D::South,
            // F
            (Piece::SouthEast, D::North) => D::East,
            (Piece::SouthEast, D::West) => D::South,
            // _ => panic!("Invalid move: {piece:?} {direction:?}"),
            _ => return None,
        };

        Some((self.step(d)?, d))
    }
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    /// The heading when moving between two adjacent spaces
    fn heading(from: Coord, to: Coord) -> Self {
        if from.0 > to.0 {
            return Direction::West;
        } else if from.0 < to.0 {
            return Direction::East;
        } else if from.1 > to.1 {
            return Direction::North;
        } else if from.1 < to.1 {
            return Direction::South;
        } else {
            panic!("no direction {from:?} {to:?}");
        }
    }
}

/// Print a colorized version of the grid, to see the path
fn print_grid(grid: &[Vec<Piece>], path: &[Coord], enclosed: &[Coord]) {
    use colored::Colorize;

    for (y, line) in grid.iter().enumerate() {
        println!(
            "{}",
            line.iter()
                .enumerate()
                .map(|(x, p)| {
                    let coord = Coord(x, y);
                    let mut p = p.to_string().normal();
                    if enclosed.contains(&coord) && path.contains(&coord) {
                        p = p.on_red();
                    } else if path.contains(&coord) {
                        p = p.blue();
                        if path[0..2].contains(&coord) {
                            p = p.on_white();
                        }
                    } else if enclosed.contains(&coord) {
                        p = ".".on_green();
                    } else {
                        p = ".".normal();
                    }

                    p
                })
                .join("")
        );
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Ground => ".",
            Self::Start => "S",
            Self::Vertical => "|",
            Self::Horizontal => "-",
            Self::NorthEast => "┖",
            Self::NorthWest => "┛",
            Self::SouthEast => "┍",
            Self::SouthWest => "┑",
        })
    }
}

impl FromStr for Piece {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "." => Self::Ground,
            "S" => Self::Start,
            "|" => Self::Vertical,
            "-" => Self::Horizontal,
            "L" => Self::NorthEast,
            "J" => Self::NorthWest,
            "F" => Self::SouthEast,
            "7" => Self::SouthWest,
            _ => anyhow::bail!("Invalid tile: {s}"),
        })
    }
}
