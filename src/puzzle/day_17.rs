use crate::{Direction, Grid, Pos};

use super::Puzzle;

pub struct Day17;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Path<'a> {
    path: Vec<Pos>,
    grid: &'a Grid<u8>,
    goal: Pos,
}

impl Puzzle for Day17 {
    fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
        Box::new(Self)
    }

    fn part_one(&self, input: &str) -> super::PuzzleResult {
        let input: Grid<u8> = input.parse().unwrap();
        let size = input.size();
        let start = Pos::ZERO;
        let end = Pos::from((size.x - 1, size.y - 1));

        let path = Path {
            path: vec![start],
            grid: &input,
            goal: end,
        };
        pathfinding::directed::astar::astar(&path, Path::astar_succ, Path::cost, Path::is_complete);

        todo!()
    }

    fn part_two(&self, _input: &str) -> super::PuzzleResult {
        todo!("implement part two")
    }
}

impl<'a> Path<'a> {
    fn cost(&self) -> usize {
        if self.path.len() == 0 {
            return 0;
        }
        self.path[1..]
            .iter()
            .map(|pos| *self.grid.value(pos).unwrap() as usize)
            .sum::<usize>()
    }

    fn is_complete(&self) -> bool {
        *self.path.last().unwrap() == self.goal
    }

    fn astar_succ(&self) -> Vec<(Self, usize)> {
        todo!()
    }
}
