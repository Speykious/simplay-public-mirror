#![allow(dead_code)]

#[derive(Debug, PartialEq)]
pub enum AxisCoord {
    X,
    Y,
    Z,
}

#[derive(PartialEq, Debug, Clone, Copy, Hash, Eq)]
pub enum Axis {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

impl Axis {
    pub fn vec_all() -> Vec<Axis> {
        return vec![
            Axis::North,
            Axis::South,
            Axis::East,
            Axis::West,
            Axis::Up,
            Axis::Down,
        ];
    }

    pub fn greedy_mesh_traverse_coords(&self) -> [AxisCoord; 2] {
        return match self {
            Self::North => [AxisCoord::X, AxisCoord::Y],
            Self::South => [AxisCoord::X, AxisCoord::Y],
            Self::East => [AxisCoord::Y, AxisCoord::Z],
            Self::West => [AxisCoord::Y, AxisCoord::Z],
            Self::Up => [AxisCoord::X, AxisCoord::Z],
            Self::Down => [AxisCoord::X, AxisCoord::Z],
        };
    }

    pub fn axis_direction(&self) -> AxisCoord {
        return match self {
            Self::North => AxisCoord::Z,
            Self::South => AxisCoord::Z,
            Self::East => AxisCoord::X,
            Self::West => AxisCoord::X,
            Self::Up => AxisCoord::Y,
            Self::Down => AxisCoord::Y,
        };
    }

    pub fn coord_offset(&self) -> (i8, i8, i8) {
        return match self {
            Self::North => (0, 0, -1),
            Self::South => (0, 0, 1),
            Self::East => (1, 0, 0),
            Self::West => (-1, 0, 0),
            Self::Up => (0, 1, 0),
            Self::Down => (0, -1, 0),
        };
    }

    pub fn coord_offset_from(&self, x: i16, y: i16, z: i16) -> (i8, i8, i8) {
        let ac = self.coord_offset();
        let sc = (x, y, z);
        let rc = (sc.0 as i8 + ac.0, sc.1 as i8 + ac.1, sc.2 as i8 + ac.2);

        return rc;
    }
}
