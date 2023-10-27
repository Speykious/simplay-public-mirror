#![allow(dead_code)]

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

impl Direction {
    pub fn offset(&self) -> (i8, i8, i8) {
        return match self {
            Self::North => (0, 0, -1),
            Self::South => (0, 0, 1),
            Self::East => (1, 0, 0),
            Self::West => (-1, 0, 0),
            Self::Up => (0, 1, 0),
            Self::Down => (0, -1, 0),
        };
    }

    pub fn offset_with_position(&self, position: (isize, isize, isize)) -> (isize, isize, isize) {
        let o = self.offset();

        let o = (o.0 as isize, o.1 as isize, o.2 as isize);

        return (position.0 + o.0, position.1 + o.1, position.2 + o.2);
    }

    pub fn all() -> Vec<Self> {
        return vec![
            Self::North,
            Self::South,
            Self::East,
            Self::West,
            Self::Up,
            Self::Down,
        ];
    }
}
