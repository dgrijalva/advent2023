use itertools::Itertools;

use crate::{Direction, Grid, Pos};

pub struct Path(pub Vec<PathTile>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PathTile {
    pub pos: Pos,
    pub shape: PathShape,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PathShape {
    Ground,
    Vertical,
    Horizontal,
    NorthEast, // ┖
    NorthWest, // ┛
    SouthEast, // ┍
    SouthWest, // ┑
}

impl Path {
    /// Find a path on a grid
    pub fn from_grid<T>(grid: &Grid<T>, on_path: impl Fn(&Grid<T>, &Pos) -> bool) -> Option<Self> {
        let size = grid.size();
        let mut path = vec![];

        // Walk in from the left to find an edge
        let mut start = None;
        'Y: for y in 0..size.y {
            for x in 0..size.x {
                let pos = Pos { x, y };
                if on_path(grid, &pos) {
                    start = Some(pos);
                    break 'Y;
                }
            }
        }
        let Some(start) = start else {
            return None;
        };
        let mut pos = start;
        let mut shape = Self::path_shape(grid, start, &on_path);
        // start walking clockwise
        let mut dir = match shape {
            PathShape::Ground => panic!("invalid starting pos"),
            PathShape::Vertical => Direction::North,
            PathShape::Horizontal => Direction::East,
            PathShape::NorthEast => Direction::North,
            PathShape::NorthWest => Direction::North,
            PathShape::SouthEast => Direction::East,
            PathShape::SouthWest => Direction::South,
        };
        path.push(PathTile { pos, shape });

        type D = Direction;
        type S = PathShape;

        loop {
            pos = grid.step(&pos, dir).expect("valid step");
            shape = Self::path_shape(grid, pos, &on_path);
            dir = match (dir, shape) {
                (D::West | D::East, S::Horizontal) => dir,
                (D::North | D::South, S::Vertical) => dir,

                (D::South, S::NorthEast) => D::East,
                (D::West, S::NorthEast) => D::North,
                (D::South, S::NorthWest) => D::West,
                (D::East, S::NorthWest) => D::North,

                (D::North, S::SouthEast) => D::East,
                (D::West, S::SouthEast) => D::South,
                (D::North, S::SouthWest) => D::West,
                (D::East, S::SouthWest) => D::South,

                _ => panic!("invalid step {pos:?} {dir:?} {shape:?}"),
            };
            let tile = PathTile { pos, shape };

            if path.contains(&tile) {
                break;
            }
            path.push(tile);
        }

        Some(Path(path))
    }

    pub fn path_shape<T>(
        grid: &Grid<T>,
        pos: Pos,
        on_path: impl Fn(&Grid<T>, &Pos) -> bool,
    ) -> PathShape {
        type D = Direction;

        let directions = Direction::all()
            .into_iter()
            .filter(|&d| {
                let Some(pos) = grid.step(&pos, d) else {
                    return false;
                };
                on_path(grid, &pos)
            })
            .collect_vec();

        if directions.is_empty() {
            return PathShape::Ground;
        }
        if directions.len() == 1 {
            return match directions[0] {
                D::South | D::North => PathShape::Vertical,
                D::East | D::West => PathShape::Horizontal,
            };
        }

        let check_shape = |a: D, b: D| -> bool { directions.iter().all(|&d| d == a || d == b) };

        if directions.len() == 2 {
            if check_shape(D::North, D::South) {
                return PathShape::Vertical;
            }
            if check_shape(D::West, D::East) {
                return PathShape::Horizontal;
            }

            if check_shape(D::North, D::East) {
                return PathShape::NorthEast;
            }
            if check_shape(D::North, D::West) {
                return PathShape::NorthWest;
            }

            if check_shape(D::South, D::East) {
                return PathShape::SouthEast;
            }
            if check_shape(D::South, D::West) {
                return PathShape::SouthWest;
            }
        }

        unreachable!("invalid shape {directions:?}");
    }

    pub fn walk(&self) -> impl Iterator<Item = (Pos, Direction)> + '_ {
        self.0.iter().enumerate().map(|(idx, tile)| {
            let mut nidx = idx + 1;
            if nidx == self.0.len() {
                nidx = 0;
            }

            (tile.pos, Direction::heading(tile.pos, self.0[nidx].pos))
        })
    }
}

impl std::fmt::Display for PathShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Ground => ".",
            Self::Vertical => "|",
            Self::Horizontal => "-",
            Self::NorthEast => "┖",
            Self::NorthWest => "┛",
            Self::SouthEast => "┍",
            Self::SouthWest => "┑",
        })
    }
}
