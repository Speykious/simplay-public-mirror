#![allow(dead_code)]

#[derive(PartialEq)]
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
}
