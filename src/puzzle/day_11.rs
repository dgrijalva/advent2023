use super::Puzzle;
use itertools::Itertools;

pub struct Day11;

#[derive(Debug, PartialEq, Eq)]
struct Coord(usize, usize);

impl Puzzle for Day11 {
    fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
        Box::new(Self)
    }

    fn part_one(&self, input: &str) -> super::PuzzleResult {
        let galaxies = parse_input(input, 2);
        println!("{:#?}", galaxies);
        let result = galaxies
            .iter()
            .combinations(2)
            .map(|p| p[0].dist(p[1]))
            .sum::<usize>();

        Ok(result.to_string())
    }

    fn part_two(&self, input: &str) -> super::PuzzleResult {
        let galaxies = parse_input(input, 1000000);
        println!("{:#?}", galaxies);
        let result = galaxies
            .iter()
            .combinations(2)
            .map(|p| p[0].dist(p[1]))
            .sum::<usize>();

        Ok(result.to_string())
    }
}

impl Coord {
    fn dist(&self, other: &Self) -> usize {
        let x_dist = self.0.max(other.0) - self.0.min(other.0);
        let y_dist = self.1.max(other.1) - self.1.min(other.1);
        x_dist + y_dist
    }
}

/// Just returns the list of galaxies
fn parse_input(input: &str, expansion: usize) -> Vec<Coord> {
    // 2D array of bools. `true` means there is a galaxy
    let grid = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| line.chars().map(|c| c == '#').collect_vec())
        .collect_vec();

    let empty_rows = grid
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&v| v == false))
        .map(|(y, _)| y)
        .collect_vec();

    let width = grid[0].len();
    let empty_cols = (0..width)
        .into_iter()
        .filter(|&i| grid.iter().all(|row| row[i] == false))
        .collect_vec();

    let galaxies = grid
        .iter()
        .enumerate()
        .map(|(y, row)| {
            let empty_row_count = empty_rows.iter().filter(|z| **z < y).count();
            let y = y + (empty_row_count * expansion) - empty_row_count;
            row.iter()
                .enumerate()
                .filter(|(_, &v)| v)
                .map(|(x, _)| {
                    let empty_col_count = empty_cols.iter().filter(|z| **z < x).count();
                    let x = x + (empty_col_count * expansion) - empty_col_count;
                    Coord(x, y)
                })
                .collect_vec()
        })
        .flatten()
        .collect_vec();

    galaxies
}
