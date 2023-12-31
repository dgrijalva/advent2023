use std::{fmt::Debug, str::FromStr};

use itertools::Itertools;

use crate::{Direction, Grid, Path, Pos};

use super::Puzzle;

pub struct Day18;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
struct Color([u8; 3]);

struct Instruction {
    dir: Direction,
    dist: usize,
    color: Color,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum Tile {
    #[default]
    Ground,
    Hole(Color),
}

impl Puzzle for Day18 {
    fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
        Box::new(Self)
    }

    fn part_one(&self, input: &str) -> super::PuzzleResult {
        let input = parse_input(input);
        let mut grid: Grid<Tile> = Grid::new(1, 1, Tile::Hole(Color::default()));
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
                grid.set(&pos, Tile::Hole(step.color));
            }
        }

        // walk the path and flood fill to the right
        let path =
            Path::from_grid(&grid, |g, p| matches!(g.value(p), Some(Tile::Hole(_)))).unwrap();

        for (pos, dir) in path.walk() {
            let Some(right_pos) = grid.step(&pos, dir.turn_right()) else {
                continue;
            };
            let cv = grid.value(&right_pos).copied();
            if matches!(cv, Some(Tile::Ground)) {
                grid.flood_fill(right_pos, Tile::Hole(Color::default()), |g, p| {
                    matches!(g.value(p), Some(Tile::Hole(_)))
                })
            }
        }

        // println!("{:?}", grid);

        let result = grid
            .scan()
            .filter(|p| matches!(grid.value(p), Some(Tile::Hole(_))))
            .count();
        Ok(result.to_string())
    }

    fn part_two(&self, _input: &str) -> super::PuzzleResult {
        todo!("implement part two")
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect_vec()
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ground => f.write_str("."),
            Self::Hole(_) => f.write_str("#"),
        }
    }
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
        let color = Color::default(); // TODO
        Ok(Instruction { dir, dist, color })
    }
}
