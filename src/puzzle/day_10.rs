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

    fn part_two(&self, _input: &str) -> super::PuzzleResult {
        todo!("implement part two")
    }
}

fn find_loop(start: Coord, grid: &[Vec<Piece>]) -> Vec<Piece> {
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

fn walk(start: Coord, mut dir: Direction, grid: &[Vec<Piece>]) -> Option<Vec<Piece>> {
    let mut visited = HashSet::new();

    let mut piece = start.lookup(grid)?;
    let mut path = vec![piece];

    // Reset visit history
    visited.clear();
    visited.insert(start);

    // Start by walking one unit in dir
    let mut current = start.walk(dir);
    visited.insert(current);
    loop {
        piece = current.lookup(grid)?;
        if piece == Piece::Start {
            return Some(path);
        }
        path.push(piece);
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

impl Coord {
    /// Get piece located at coord, if valid
    fn lookup(&self, grid: &[Vec<Piece>]) -> Option<Piece> {
        grid.get(self.1).and_then(|row| row.get(self.0)).cloned()
    }

    /// Walk in a direction
    fn walk(&self, direction: Direction) -> Coord {
        match direction {
            Direction::North => Coord(self.0, self.1 - 1),
            Direction::South => Coord(self.0, self.1 + 1),
            Direction::West => Coord(self.0 - 1, self.1),
            Direction::East => Coord(self.0 + 1, self.1),
        }
    }

    /// Follow a pipe. Returns next coord and output direction
    fn follow(&self, piece: Piece, direction: Direction) -> Option<(Coord, Direction)> {
        type D = Direction;
        match (piece, direction) {
            (Piece::Ground, _) => None,
            (Piece::Horizontal, D::East) => Some((self.walk(D::East), D::East)),
            (Piece::Horizontal, D::West) => Some((self.walk(D::West), D::West)),
            (Piece::Vertical, D::North) => Some((self.walk(D::North), D::North)),
            (Piece::Vertical, D::South) => Some((self.walk(D::South), D::South)),
            // L
            (Piece::NorthEast, D::South) => Some((self.walk(D::East), D::East)),
            (Piece::NorthEast, D::West) => Some((self.walk(D::North), D::North)),
            // J
            (Piece::NorthWest, D::South) => Some((self.walk(D::West), D::West)),
            (Piece::NorthWest, D::East) => Some((self.walk(D::North), D::North)),
            // 7
            (Piece::SouthWest, D::North) => Some((self.walk(D::West), D::West)),
            (Piece::SouthWest, D::East) => Some((self.walk(D::South), D::South)),
            // F
            (Piece::SouthEast, D::North) => Some((self.walk(D::East), D::East)),
            (Piece::SouthEast, D::West) => Some((self.walk(D::South), D::South)),
            // _ => panic!("Invalid move: {piece:?} {direction:?}"),
            _ => return None,
        }
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
