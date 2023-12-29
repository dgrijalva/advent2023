use std::hash::Hash;

use super::Puzzle;
use crate::{Direction, Grid, Pos};

pub struct Day17;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
struct Path<'a> {
    pos: Pos,
    heading: Direction,
    run_len: u8,
    grid: &'a Grid<u8>,
    goal: Pos,
    crucible: Crucible,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Crucible {
    Normal,
    Ultra,
}

impl Puzzle for Day17 {
    fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
        Box::new(Self)
    }

    fn part_one(&self, input: &str) -> super::PuzzleResult {
        solve_puzzle(input, Crucible::Normal)
    }

    fn part_two(&self, input: &str) -> super::PuzzleResult {
        solve_puzzle(input, Crucible::Ultra)
    }
}

fn solve_puzzle(input: &str, crucuble: Crucible) -> super::PuzzleResult {
    let input: Grid<u8> = input.parse().unwrap();
    let size = input.size();
    let start = Pos::ZERO;
    let end = Pos::from((size.x - 1, size.y - 1));

    let path = Path {
        pos: start,
        heading: Direction::East,
        run_len: 0,
        grid: &input,
        goal: end,
        crucible: crucuble,
    };
    let Some((_, result)) = pathfinding::directed::astar::astar(
        &path,
        Path::astar_succ,
        Path::guess_cost_to_end,
        Path::is_complete,
    ) else {
        anyhow::bail!("no path found");
    };

    Ok(result.to_string())
}

impl<'a> Path<'a> {
    fn curr_cost(&self) -> usize {
        *self.grid.value(&self.pos).unwrap() as usize
    }

    fn guess_cost_to_end(&self) -> usize {
        let curr_pos = self.pos;
        let size = self.grid.size();
        let x_dist = size.x - curr_pos.x;
        let y_dist = size.y - curr_pos.y;
        f64::sqrt(((x_dist * x_dist) + (y_dist * y_dist)) as f64) as usize
    }

    fn is_complete(&self) -> bool {
        // Can only stop after at least 4 blocks
        if self.crucible == Crucible::Ultra && (self.run_len < 4 || self.run_len > 10) {
            return false;
        }

        self.pos == self.goal
    }

    fn step(&self, d: Direction) -> Option<Self> {
        let next_pos = self.grid.step(&self.pos, d)?;
        let new_heading = Direction::heading(self.pos, next_pos);
        let mut next = self.clone();
        next.pos = next_pos;
        next.heading = new_heading;
        if next.heading == self.heading {
            next.run_len += 1;
        } else {
            next.run_len = 1;
        }

        // validate
        match self.crucible {
            Crucible::Normal => {
                // no more than 3 blocks without a turn
                if next.run_len > 3 {
                    return None;
                }
            }
            Crucible::Ultra => {
                // can only turn after between 4-10 blocks
                if self.heading != next.heading && (self.run_len < 4 || self.run_len > 10) {
                    return None;
                }
            }
        }

        Some(next)
    }

    fn astar_succ(&self) -> Vec<(Self, usize)> {
        let mut succ = Vec::with_capacity(4);
        let heading = self.heading;
        for d in [heading, heading.turn_right(), heading.turn_left()] {
            let Some(next) = self.step(d) else {
                continue;
            };
            let cost = next.curr_cost();
            succ.push((next, cost));
        }
        // for (p, c) in &succ {
        //     let dp: Vec<(usize, usize)> = p.path.iter().copied().map(Into::into).collect();
        //     println!("{:?} -> {}", dp, c);
        // }

        succ
    }
}

impl<'a> Hash for Path<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
        self.run_len.hash(state);
        self.heading.hash(state);
    }
}
