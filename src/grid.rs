use crate::{Direction, Pos};
use itertools::Itertools;
use std::str::FromStr;

pub struct Grid<T>(Vec<Vec<T>>);

impl<T> Grid<T> {
    pub fn size(&self) -> Pos {
        Pos::size_of(&self.0)
    }

    pub fn value(&self, pos: &Pos) -> Option<&T> {
        self.0.get(pos.y)?.get(pos.x)
    }

    pub fn set(&mut self, pos: &Pos, value: T) {
        self.0[pos.y][pos.x] = value;
    }

    pub fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> + '_ {
        self.0.iter().map(|r| r.iter())
    }

    pub fn row(&self, idx: usize) -> impl Iterator<Item = &T> {
        self.0[idx].iter()
    }

    pub fn col(&self, idx: usize) -> impl Iterator<Item = &T> + '_ {
        self.0.iter().map(move |row| &row[idx])
    }

    pub fn scan(&self) -> impl Iterator<Item = Pos> {
        let size = self.size();
        (0..size.y)
            .map(move |y| (0..size.x).map(move |x| Pos::from((x, y))))
            .flatten()
    }

    pub fn step(&self, pos: &Pos, direction: Direction) -> Option<Pos> {
        let size = self.size();
        match direction {
            Direction::North => (pos.y > 0).then(|| Pos {
                x: pos.x,
                y: pos.y - 1,
            }),
            Direction::East => (pos.x < size.x - 1).then(|| Pos {
                x: pos.x + 1,
                y: pos.y,
            }),
            Direction::South => (pos.y < size.y - 1).then(|| Pos {
                x: pos.x,
                y: pos.y + 1,
            }),
            Direction::West => (pos.x > 0).then(|| Pos {
                x: pos.x - 1,
                y: pos.y,
            }),
        }
    }

    pub fn debug_print(&self, f: impl Fn(Pos, &T) -> String) {
        for (y, row) in self.rows().enumerate() {
            let data = row
                .enumerate()
                .map(|(x, t)| f(Pos::from((x, y)), &t))
                .join("");
            log::debug!("{data}");
        }
    }
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn new(width: usize, height: usize, value: T) -> Self {
        Grid(
            (0..height)
                .map(|_| std::iter::repeat(value.clone()).take(width).collect_vec())
                .collect_vec(),
        )
    }
}

impl<T> Clone for Grid<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> FromStr for Grid<T>
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse().unwrap())
                    .collect_vec()
            })
            .collect_vec();
        Ok(Self(data))
    }
}
