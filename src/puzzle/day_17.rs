use std::{cell::RefCell, collections::HashSet, hash::Hash, rc::Rc};

use itertools::Itertools;

use crate::{Direction, Grid, Pos};

use super::Puzzle;

pub struct Day17;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Path<'a> {
    path: Vec<Pos>,
    grid: &'a Grid<u8>,
    goal: Pos,
    memo: Rc<RefCell<HashSet<Vec<Pos>>>>,
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
        let memo = HashSet::default();

        let path = Path {
            path: vec![start],
            grid: &input,
            goal: end,
            memo: Rc::new(RefCell::new(memo)),
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

    fn part_two(&self, _input: &str) -> super::PuzzleResult {
        todo!("implement part two")
    }
}

impl<'a> Path<'a> {
    fn curr_cost(&self) -> usize {
        let pos = self.path.last().unwrap();
        *self.grid.value(pos).unwrap() as usize
    }

    fn cost(&self) -> usize {
        // no heat loss on starting tile
        if self.path.len() < 2 {
            return 0;
        }
        self.path[1..]
            .iter()
            .map(|pos| *self.grid.value(pos).unwrap() as usize)
            .sum::<usize>()
    }

    fn guess_cost_to_end(&self) -> usize {
        let curr_pos = self.path.last().unwrap();
        let size = self.grid.size();
        let x_dist = size.x - curr_pos.x;
        let y_dist = size.y - curr_pos.y;
        f64::sqrt(((x_dist * x_dist) + (y_dist * y_dist)) as f64) as usize
    }

    fn is_complete(&self) -> bool {
        *self.path.last().unwrap() == self.goal
    }

    fn heading(&self) -> Direction {
        if self.path.len() < 2 {
            return Direction::East;
        }
        Direction::heading(
            self.path[self.path.len() - 2],
            self.path[self.path.len() - 1],
        )
    }

    fn valid(&self) -> bool {
        // no backtracking
        if self.path.len() > 1 {
            let mut pr = self.path.iter().rev();
            if pr.next() == pr.next() {
                println!("no backtracking");
                return false;
            }
        }

        // No dupes
        if self.memo.borrow().contains(&self.path) {
            println!("cache hit");
            return false;
        }

        // can't move in the same direction more than 3 tiles
        // [a b c d e] pos
        // [ 1 2 3 4 ] dir
        if self.path.len() > 4 {
            let check = &self.path[self.path.len() - 5..];
            let dir = Direction::heading(check[0], check[1]);
            if [
                (check[1], check[2]),
                (check[2], check[3]),
                (check[3], check[4]),
            ]
            .iter()
            .all(|(a, b)| Direction::heading(*a, *b) == dir)
            {
                let dp: Vec<(usize, usize)> = self.path.iter().copied().map_into().collect();
                // println!("long run: {:?}", dp);
                self.memo.borrow_mut().insert(self.path.clone());
                return false;
            }
        }

        true
    }

    fn astar_succ(&self) -> Vec<(Self, usize)> {
        let pos = self.path.last().unwrap();
        let mut succ = Vec::with_capacity(4);
        let heading = self.heading();
        for d in [heading, heading.turn_right(), heading.turn_left()] {
            let Some(next) = self.grid.step(&pos, d) else {
                continue;
            };
            let mut new_path = self.clone();
            new_path.path.push(next);
            if new_path.valid() {
                let cost = new_path.curr_cost();
                self.memo.borrow_mut().insert(new_path.path.clone());
                succ.push((new_path, cost));
            }
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
        self.path.hash(state)
    }
}
