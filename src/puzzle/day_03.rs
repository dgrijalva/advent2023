//! This is a template for a puzzle solution.  Copy this file to a new file.
//! Files in this folder are auto-discovered at build time.

use super::Puzzle;
use std::str::FromStr;

pub struct Day03;

struct Schematic(Vec<Vec<Datum>>);
#[derive(Debug, Clone, Copy)]
struct Pos(usize, usize);

#[derive(Debug, Clone, Copy)]
enum Datum {
    None,
    Symbol(char),
    Number(u8),
}

impl Puzzle for Day03 {
    fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
        Box::new(Self)
    }

    fn part_one(&self, input: &str) -> super::PuzzleResult {
        let data: Schematic = input.parse().unwrap();
        let numbers = data
            .numbers()
            .filter(|(_, pos, len)| data.has_adjacent_symbols(*pos, *len))
            .map(|(n, _, _)| n)
            .sum::<usize>();
        Ok(numbers.to_string())
    }

    fn part_two(&self, _input: &str) -> super::PuzzleResult {
        todo!("implement part two")
    }
}

impl Day03 {}

impl Schematic {
    /// Returns an iterator of (number, position, len) for all matches
    fn numbers(&self) -> impl Iterator<Item = (usize, Pos, usize)> + '_ {
        self.0.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .fold(NumberAccumulator::default(), |mut acc, (x, datum)| {
                    acc.push(Pos(x, y), *datum);
                    acc
                })
                .complete()
        })
    }

    fn has_adjacent_symbols(&self, pos: Pos, len: usize) -> bool {
        let Pos(px, py) = pos;
        let px2 = px + len - 1;

        let left = if px > 0 { px - 1 } else { px };
        let right = if px2 + 1 < self.0[py].len() {
            px2 + 1
        } else {
            px2
        };

        // Above
        if py > 0 {
            let above = &self.0[py - 1][left..=right];
            if above.iter().any(|d| matches!(d, Datum::Symbol(_))) {
                return true;
            }
        }

        // Before
        if px != left && matches!(self.0[py][left], Datum::Symbol(_)) {
            return true;
        }

        // After
        if px != right && matches!(self.0[py][right], Datum::Symbol(_)) {
            return true;
        }

        // Below
        if py + 1 < self.0.len() {
            let below = &self.0[py + 1][left..=right];
            if below.iter().any(|d| matches!(d, Datum::Symbol(_))) {
                return true;
            }
        }

        false
    }
}

#[derive(Debug, Default)]
struct NumberAccumulator {
    complete: Vec<(usize, Pos, usize)>,
    current: Option<(usize, Pos, usize)>,
}

impl NumberAccumulator {
    fn push(&mut self, pos: Pos, datum: Datum) {
        match datum {
            Datum::Number(n) => {
                if let Some((number, _, len)) = &mut self.current {
                    *number = *number * 10 + n as usize;
                    *len = *len + 1;
                } else {
                    self.current = Some((n as usize, pos, 1));
                }
            }
            Datum::Symbol(_) | Datum::None => {
                self.commit();
            }
        }
    }

    fn commit(&mut self) {
        if let Some(current) = self.current.take() {
            self.complete.push(current);
        }
    }

    fn complete(&mut self) -> Vec<(usize, Pos, usize)> {
        if let Some(current) = self.current.take() {
            self.complete.push(current);
        }
        self.complete.clone()
    }
}

impl FromStr for Schematic {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Schematic(
            input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => Datum::None,
                            _ if c.is_numeric() => Datum::Number(c.to_digit(10).unwrap() as u8),
                            _ => Datum::Symbol(c),
                        })
                        .collect()
                })
                .collect(),
        ))
    }
}
