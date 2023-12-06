use super::Puzzle;
use std::collections::HashMap;
use std::str::FromStr;

pub struct Day03;

struct Schematic(Vec<Vec<Datum>>);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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
            .filter(|(_, pos, len)| !data.adjacent_symbols(*pos, *len).is_empty())
            .map(|(n, _, _)| n)
            .sum::<usize>();
        Ok(numbers.to_string())
    }

    fn part_two(&self, input: &str) -> super::PuzzleResult {
        let data: Schematic = input.parse().unwrap();
        let pairs: HashMap<Pos, Vec<usize>> = data
            .numbers()
            // Find all the numbers that have a star adjacent to them
            // Procduces a tuple of (star_pos, number)
            .filter_map(|(num, pos, len)| {
                data.adjacent_symbols(pos, len)
                    .into_iter()
                    .find(|(_, d)| matches!(d, Datum::Symbol('*')))
                    .map(|(star_pos, _)| (star_pos, num))
            })
            // Group the numbers by the position of the star
            .fold(HashMap::new(), |mut acc, (pos, num)| {
                acc.entry(pos).or_default().push(num);
                acc
            });

        // For all the groups with exactly two numbers, multiply them together
        // and sum the results
        let result = pairs.into_values()
            .filter(|nums| nums.len() == 2)
            .map(|nums| nums[0] * nums[1])
            .sum::<usize>();

        Ok(result.to_string())
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

    /// Takes the starting position and length and looks at all the adjacent
    /// cells to see if there are any symbols. Returns a list of what it finds.
    fn adjacent_symbols(&self, pos: Pos, len: usize) -> Vec<(Pos, Datum)> {
        let Pos(px, py) = pos;
        let px2 = px + len - 1;

        let left = if px > 0 { px - 1 } else { px };
        let right = if px2 + 1 < self.0[py].len() {
            px2 + 1
        } else {
            px2
        };

        let mut symbols = Vec::new();

        // Above
        if py > 0 {
            let y = py - 1;
            symbols.extend(
                (left..=right)
                    .map(|x| (Pos(x, y), self.0[y][x]))
                    .filter(|(_, d)| matches!(d, Datum::Symbol(_))),
            );
        }

        // Before
        if px != left && matches!(self.0[py][left], Datum::Symbol(_)) {
            symbols.push((Pos(left, py), self.0[py][left]));
        }

        // After
        if px != right && matches!(self.0[py][right], Datum::Symbol(_)) {
            symbols.push((Pos(right, py), self.0[py][right]));
        }

        // Below
        if py + 1 < self.0.len() {
            let y = py + 1;
            symbols.extend(
                (left..=right)
                    .map(|x| (Pos(x, y), self.0[y][x]))
                    .filter(|(_, d)| matches!(d, Datum::Symbol(_))),
            );
        }

        symbols
    }
}

/// Each position in the data holds a single digit. This is used to accumulate
/// the digits into the number it's meant to represent, as we scan over a row.
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
                    *len += 1;
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
        self.commit();
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
