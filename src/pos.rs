#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub const ZERO: Pos = Pos { x: 0, y: 0 };

    pub fn size_of<T>(data: &Vec<Vec<T>>) -> Self {
        Pos {
            y: data.len(),
            x: data[0].len(),
        }
    }
}

impl From<(usize, usize)> for Pos {
    fn from(value: (usize, usize)) -> Self {
        Pos {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<Pos> for (usize, usize) {
    fn from(value: Pos) -> Self {
        (value.x, value.y)
    }
}
