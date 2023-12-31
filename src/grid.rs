use crate::{Direction, Pos};
use itertools::Itertools;
use std::{hash::Hash, str::FromStr};

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

    pub fn walk(&self, pos: &Pos, direction: Direction, dist: usize) -> Option<Pos> {
        let mut pos = *pos;
        for _ in 0..dist {
            pos = self.step(&pos, direction)?;
        }
        Some(pos)
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

    pub fn step_unchecked(&self, pos: &Pos, direction: Direction) -> Pos {
        match direction {
            Direction::North => Pos {
                x: pos.x,
                y: pos.y - 1,
            },
            Direction::East => Pos {
                x: pos.x + 1,
                y: pos.y,
            },
            Direction::South => Pos {
                x: pos.x,
                y: pos.y + 1,
            },
            Direction::West => Pos {
                x: pos.x - 1,
                y: pos.y,
            },
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

impl<T> Grid<T>
where
    T: Clone + PartialEq,
{
    pub fn flood_fill(&mut self, start: Pos, value: T, is_boundary: impl Fn(&Self, &Pos) -> bool) {
        type D = Direction;
        let mut next = vec![start];
        let bounds = self.size();
        while let Some(pos) = next.pop() {
            self.set(&pos, value.clone());
            for d in [D::North, D::East, D::West, D::South] {
                let Some(p) = self.step(&pos, d) else {
                    continue;
                };
                if p.x >= bounds.x || p.y >= bounds.y {
                    continue;
                }

                if Some(&value) == self.value(&p) {
                    continue;
                }
                if is_boundary(&self, &p) {
                    continue;
                }
                next.push(p);
            }
        }
    }
}

impl<T> Grid<T>
where
    T: Default + Clone,
{
    /// Expands and returns new size
    pub fn expand(&mut self, width: usize, height: usize) -> Pos {
        self.make_space(&Pos::ZERO, Direction::East, width);
        self.make_space(&Pos::ZERO, Direction::South, height);

        self.size()
    }

    /// Ensure the grid has enough space in the target direction
    /// Returns new position
    pub fn make_space(&mut self, pos: &Pos, dir: Direction, dist: usize) -> Pos {
        // already ok
        if self.walk(pos, dir, dist).is_some() {
            return *pos;
        }

        // expand in the specified direction
        let size = self.size();
        let mut pos = *pos;
        match dir {
            Direction::South => {
                let dist = dist - (size.y - (pos.y + 1));
                for _ in 0..dist {
                    self.0.push(vec![T::default(); size.x]);
                }
            }
            Direction::North => {
                // push new rows onto the end, then rotate them to the front
                let dist = dist - pos.y;
                for _ in 0..dist {
                    self.0.push(vec![T::default(); size.x]);
                }
                self.0.rotate_right(dist);
                pos.y += dist;
            }
            Direction::East => {
                let dist = dist - (size.x - (pos.x + 1));
                for row in self.0.iter_mut() {
                    row.extend(std::iter::repeat(T::default()).take(dist));
                }
            }
            Direction::West => {
                let dist = dist - pos.x;
                for row in self.0.iter_mut() {
                    row.extend(std::iter::repeat(T::default()).take(dist));
                    row.rotate_right(dist);
                }
                pos.x += dist;
            }
        }

        return pos;
    }
}

impl<T> PartialEq for Grid<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Grid<T> where T: Eq {}

impl<T> Hash for Grid<T>
where
    T: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
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

impl<T> std::fmt::Debug for Grid<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows() {
            for v in row {
                std::fmt::Debug::fmt(v, f)?;
            }
            f.write_str("\n")?;
        }
        Ok(())
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
