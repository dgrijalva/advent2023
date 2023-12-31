use std::{fmt::Debug, str::FromStr};

use itertools::Itertools;

use crate::{Direction, Grid, Path, Pos};

use super::Puzzle;

pub struct Day18;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction {
    dir: Direction,
    dist: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum Tile {
    #[default]
    Ground,
    Hole,
}

impl Puzzle for Day18 {
    fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
        Box::new(Self)
    }

    fn part_one(&self, input: &str) -> super::PuzzleResult {
        let input = input
            .lines()
            .map(|line| line.parse().unwrap())
            .collect_vec();

        solve_puzzle(&input)
    }

    fn part_two(&self, input: &str) -> super::PuzzleResult {
        let input = input
            .lines()
            .map(|line| parse_input_hex(line).unwrap())
            .collect_vec();

        // for line in &input {
        //     println!("{line:?}");
        // }

        solve_puzzle(&input)
    }
}

fn solve_puzzle(input: &[Instruction]) -> super::PuzzleResult {
    let mut grid: Grid<Tile> = Grid::new(1, 1, Tile::Hole);
    let mut pos = Pos::ZERO;
    for step in input {
        pos = grid.make_space(&pos, step.dir, step.dist);
        for _ in 0..step.dist {
            pos = match grid.step(&pos, step.dir) {
                Some(p) => p,
                None => {
                    panic!(
                        "attempted to step from {pos:?} {:?} Size: {:?}",
                        step.dir,
                        grid.size()
                    );
                }
            };
            grid.set(&pos, Tile::Hole);
        }
        println!("Building grid: {:?}", grid.size());
    }

    println!("Done building grid: {:?}", grid.size());

    // walk the path and flood fill to the right
    let path = Path::from_grid(&grid, |g, p| matches!(g.value(p), Some(Tile::Hole))).unwrap();

    println!("Done building path: {:?}", path.0.len());

    for (pos, dir) in path.walk() {
        let Some(right_pos) = grid.step(&pos, dir.turn_right()) else {
            continue;
        };
        let cv = grid.value(&right_pos).copied();
        if matches!(cv, Some(Tile::Ground)) {
            grid.flood_fill(right_pos, Tile::Hole, |g, p| {
                matches!(g.value(p), Some(Tile::Hole))
            })
        }
    }

    // println!("{:?}", grid);

    let result = grid
        .scan()
        .filter(|p| matches!(grid.value(p), Some(Tile::Hole)))
        .count();
    Ok(result.to_string())
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ground => f.write_str("."),
            Self::Hole => f.write_str("#"),
        }
    }
}

fn parse_input_hex(input: &str) -> Result<Instruction, anyhow::Error> {
    let parts = input.split(' ').collect_vec();
    let hex = parts[2].replace('(', "").replace(')', "").replace('#', "");
    let dist = usize::from_str_radix(&hex[0..5], 16).unwrap();
    let dir = match u8::from_str_radix(&hex[5..], 16).unwrap() {
        0 => Direction::East,
        1 => Direction::South,
        2 => Direction::West,
        3 => Direction::North,
        d => panic!("invalid direction {d}"),
    };

    Ok(Instruction { dir, dist })
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect_vec();
        let dir = match parts[0] {
            "U" => Direction::North,
            "R" => Direction::East,
            "D" => Direction::South,
            "L" => Direction::West,
            other => anyhow::bail!("Unexpected direction: {other}"),
        };
        let dist = parts[1].parse().unwrap();
        Ok(Instruction { dir, dist })
    }
}
