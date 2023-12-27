use crate::Pos;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn turn_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    /// The heading when moving between two adjacent spaces
    pub fn heading(from: Pos, to: Pos) -> Self {
        if from.x > to.x {
            return Direction::West;
        } else if from.x < to.x {
            return Direction::East;
        } else if from.y > to.y {
            return Direction::North;
        } else if from.y < to.y {
            return Direction::South;
        } else {
            panic!("no direction {from:?} {to:?}");
        }
    }
}
