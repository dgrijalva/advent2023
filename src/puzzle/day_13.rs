use itertools::Itertools;
use std::str::FromStr;

use super::Puzzle;
use crate::Pos;

pub struct Day13;

#[derive(Clone, Debug)]
struct Grid(Vec<Vec<Location>>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Location {
    Rock,
    Ash,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Horizontal,
    Vertical,
}

impl Puzzle for Day13 {
    fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
        Box::new(Self)
    }

    fn part_one(&self, input: &str) -> super::PuzzleResult {
        let input = parse_input(input);
        let result = input
            .iter()
            .map(|grid| {
                let mut val = 0usize;
                val += grid.perfect_reflection(grid.horiz()) * 100;
                val += grid.perfect_reflection(grid.vert());
                val
            })
            .inspect(|v| println!("{v}"))
            .sum::<usize>();

        Ok(result.to_string())
    }

    fn part_two(&self, input: &str) -> super::PuzzleResult {
        let input = parse_input(input);
        let result = input
            .iter()
            .map(|grid| {
                let mut val = 0usize;
                let (dir, c) = grid.smudged_reflection();
                match dir {
                    Direction::Horizontal => val += c * 100,
                    Direction::Vertical => val += c,
                };
                val
            })
            .inspect(|v| println!("{v}"))
            .sum::<usize>();

        Ok(result.to_string())
    }
}

impl Grid {
    fn row(&self, idx: usize) -> Vec<Location> {
        self.0[idx].clone()
    }

    fn col(&self, idx: usize) -> Vec<Location> {
        self.0.iter().map(|row| row[idx]).collect_vec()
    }

    fn size(&self) -> Pos {
        Pos {
            y: self.0.len(),
            x: self.0[0].len(),
        }
    }

    fn horiz(&self) -> (impl Fn(&Grid, usize) -> Vec<Location>, usize) {
        (Self::row, self.size().y)
    }

    fn vert(&self) -> (impl Fn(&Grid, usize) -> Vec<Location>, usize) {
        (Self::col, self.size().x)
    }

    /// Returns the index of the top line of a reflection
    fn perfect_reflection(
        &self,
        direction: (impl Fn(&Grid, usize) -> Vec<Location>, usize),
    ) -> usize {
        self.find_reflections(&direction.0, direction.1)
            .iter()
            .find_map(|r| self.validate_reflection(&direction.0, direction.1, *r))
            .map(|v| v + 1) // because we count the left side of the match
            .unwrap_or(0)
    }

    fn valid_reflection_positions(
        &self,
        direction: (impl Fn(&Grid, usize) -> Vec<Location>, usize),
        d: Direction,
    ) -> Vec<Pos> {
        self.find_reflections(&direction.0, direction.1)
            .iter()
            .filter(|&r| {
                self.validate_reflection(&direction.0, direction.1, *r)
                    .is_some()
            })
            .map(|v| v + 1)
            .map(|r| match d {
                Direction::Vertical => Pos { x: 0, y: r },
                Direction::Horizontal => Pos { x: r, y: 0 },
            })
            .collect_vec()
    }

    fn reflection_location(&self, but_not: Option<Pos>) -> Option<Pos> {
        self.valid_reflection_positions(self.horiz(), Direction::Horizontal)
            .into_iter()
            .chain(
                self.valid_reflection_positions(self.vert(), Direction::Vertical)
                    .into_iter(),
            )
            .filter(|p| {
                if let Some(but_not) = but_not.as_ref() {
                    p != but_not
                } else {
                    true
                }
            })
            .next()
    }

    /// Brute force through the grid, trying one swap until we find an alternate valid reflection
    fn smudged_reflection(&self) -> (Direction, usize) {
        let orig = self.reflection_location(None).unwrap();

        for (row_i, row) in self.0.iter().enumerate() {
            for (col_i, _) in row.iter().enumerate() {
                let mut new_grid = self.clone();
                new_grid.0[row_i][col_i] = !new_grid.0[row_i][col_i];
                let Some(new_pos) = new_grid.reflection_location(Some(orig)) else {
                    continue;
                };
                if new_pos.x > 0 {
                    return (Direction::Horizontal, new_pos.x);
                }
                if new_pos.y > 0 {
                    return (Direction::Vertical, new_pos.y);
                }
            }
        }

        unreachable!("No result found");
    }

    fn find_reflections(
        &self,
        lookup: impl Fn(&Grid, usize) -> Vec<Location>,
        len: usize,
    ) -> Vec<usize> {
        let mut reflections = vec![];
        for i in 0..(len - 1) {
            if lookup(self, i) == lookup(self, i + 1) {
                reflections.push(i);
            }
        }
        reflections
    }

    fn validate_reflection(
        &self,
        lookup: impl Fn(&Grid, usize) -> Vec<Location>,
        len: usize,
        idx: usize,
    ) -> Option<usize> {
        if idx == 0 {
            return Some(idx);
        }

        let mut left = idx - 1;
        let mut right = idx + 2;

        while right < len {
            if lookup(self, left) != lookup(self, right) {
                return None;
            }

            if left > 0 {
                left -= 1;
                right += 1;
            } else {
                break;
            }
        }

        Some(idx)
    }
}

fn parse_input(input: &str) -> Vec<Grid> {
    input
        .split("\n\n")
        .map(|g| {
            Grid(
                g.lines()
                    .map(|l| {
                        l.chars()
                            .map(|c| c.to_string().parse().unwrap())
                            .collect_vec()
                    })
                    .collect_vec(),
            )
        })
        .collect_vec()
}

impl FromStr for Location {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::Ash),
            "#" => Ok(Self::Rock),
            other => Err(anyhow::anyhow!("Unexpected symbol: {other}")),
        }
    }
}

impl std::ops::Not for Location {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Ash => Self::Rock,
            Self::Rock => Self::Ash,
        }
    }
}
