use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Point(u8, u8, u8);

#[derive(Debug, Clone)]
pub struct Grid3<T> {
    d: u8,
    pub data: Vec<T>,
}

#[derive(Debug, Clone)]
pub struct GridFront<T> {
    d: u8,
    pub data: Vec<T>,
}

#[derive(Debug, Clone)]
pub struct GridRight<T> {
    d: u8,
    pub data: Vec<T>,
}

impl Point {
    pub const fn new(x: u8, y: u8, z: u8) -> Point {
        Point(x, y, z)
    }

    pub fn to_x(self, d: u8, dx: u8) -> Option<Point> {
        let x = self.0.wrapping_add(dx);
        if x < d {
            Some(Point(x, self.1, self.2))
        } else {
            None
        }
    }

    pub fn to_y(self, d: u8, dy: u8) -> Option<Point> {
        let y = self.1.wrapping_add(dy);
        if y < d {
            Some(Point(self.0, y, self.2))
        } else {
            None
        }
    }

    pub fn to_z(self, d: u8, dz: u8) -> Option<Point> {
        let z = self.2.wrapping_add(dz);
        if z < d {
            Some(Point(self.0, self.1, z))
        } else {
            None
        }
    }

    pub fn next_cell(self, d: u8, direction: u8) -> Option<Point> {
        match direction {
            0 => self.to_x(d, 1),
            1 => self.to_x(d, !0),
            2 => self.to_y(d, 1),
            3 => self.to_y(d, !0),
            4 => self.to_z(d, 1),
            5 => self.to_z(d, !0),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AxisMap {
    None,
    Map1 { from: u8, to: u8 },
    Map2 { map: [u8; 6] },
}

impl AxisMap {
    pub const fn new() -> AxisMap {
        AxisMap::None
    }

    pub fn fix(self, d1: u8, d2: u8) -> AxisMap {
        match self {
            AxisMap::None => AxisMap::Map1 { from: d1, to: d2 },
            AxisMap::Map1 { from, to } if from == d1 || from == d1 ^ 1 => {
                AxisMap::Map1 { from, to }
            }
            AxisMap::Map1 { from, to } => {
                let map = match (from, to, d1, d2) {
                    (from1, to1, from2, to2) if from1 == to1 && from2 == to2 => [0, 1, 2, 3, 4, 5],
                    (0, 0, 2, 3)
                    | (0, 0, 3, 2)
                    | (0, 0, 4, 5)
                    | (0, 0, 5, 4)
                    | (1, 1, 2, 3)
                    | (1, 1, 3, 2)
                    | (1, 1, 4, 5)
                    | (1, 1, 5, 4)
                    | (2, 3, 0, 0)
                    | (2, 3, 1, 1)
                    | (2, 3, 3, 2)
                    | (2, 3, 4, 5)
                    | (2, 3, 5, 4)
                    | (3, 2, 0, 0)
                    | (3, 2, 1, 1)
                    | (3, 2, 2, 3)
                    | (3, 2, 4, 5)
                    | (3, 2, 5, 4)
                    | (4, 5, 0, 0)
                    | (4, 5, 1, 1)
                    | (4, 5, 2, 3)
                    | (4, 5, 3, 2)
                    | (4, 5, 5, 4)
                    | (5, 4, 0, 0)
                    | (5, 4, 1, 1)
                    | (5, 4, 2, 3)
                    | (5, 4, 3, 2)
                    | (5, 4, 4, 5) => [0, 1, 3, 2, 5, 4],
                    (0, 0, 2, 4)
                    | (0, 0, 3, 5)
                    | (0, 0, 4, 3)
                    | (0, 0, 5, 2)
                    | (1, 1, 2, 4)
                    | (1, 1, 3, 5)
                    | (1, 1, 4, 3)
                    | (1, 1, 5, 2)
                    | (2, 4, 0, 0)
                    | (2, 4, 1, 1)
                    | (2, 4, 3, 5)
                    | (2, 4, 4, 3)
                    | (2, 4, 5, 2)
                    | (3, 5, 0, 0)
                    | (3, 5, 1, 1)
                    | (3, 5, 2, 4)
                    | (3, 5, 4, 3)
                    | (3, 5, 5, 2)
                    | (4, 3, 0, 0)
                    | (4, 3, 1, 1)
                    | (4, 3, 2, 4)
                    | (4, 3, 3, 5)
                    | (4, 3, 5, 2)
                    | (5, 2, 0, 0)
                    | (5, 2, 1, 1)
                    | (5, 2, 2, 4)
                    | (5, 2, 3, 5)
                    | (5, 2, 4, 3) => [0, 1, 4, 5, 3, 2],
                    (0, 0, 2, 5)
                    | (0, 0, 3, 4)
                    | (0, 0, 4, 2)
                    | (0, 0, 5, 3)
                    | (1, 1, 2, 5)
                    | (1, 1, 3, 4)
                    | (1, 1, 4, 2)
                    | (1, 1, 5, 3)
                    | (2, 5, 0, 0)
                    | (2, 5, 1, 1)
                    | (2, 5, 3, 4)
                    | (2, 5, 4, 2)
                    | (2, 5, 5, 3)
                    | (3, 4, 0, 0)
                    | (3, 4, 1, 1)
                    | (3, 4, 2, 5)
                    | (3, 4, 4, 2)
                    | (3, 4, 5, 3)
                    | (4, 2, 0, 0)
                    | (4, 2, 1, 1)
                    | (4, 2, 2, 5)
                    | (4, 2, 3, 4)
                    | (4, 2, 5, 3)
                    | (5, 3, 0, 0)
                    | (5, 3, 1, 1)
                    | (5, 3, 2, 5)
                    | (5, 3, 3, 4)
                    | (5, 3, 4, 2) => [0, 1, 5, 4, 2, 3],
                    (0, 1, 1, 0)
                    | (0, 1, 2, 2)
                    | (0, 1, 3, 3)
                    | (0, 1, 4, 5)
                    | (0, 1, 5, 4)
                    | (1, 0, 0, 1)
                    | (1, 0, 2, 2)
                    | (1, 0, 3, 3)
                    | (1, 0, 4, 5)
                    | (1, 0, 5, 4)
                    | (2, 2, 0, 1)
                    | (2, 2, 1, 0)
                    | (2, 2, 4, 5)
                    | (2, 2, 5, 4)
                    | (3, 3, 0, 1)
                    | (3, 3, 1, 0)
                    | (3, 3, 4, 5)
                    | (3, 3, 5, 4)
                    | (4, 5, 0, 1)
                    | (4, 5, 1, 0)
                    | (4, 5, 2, 2)
                    | (4, 5, 3, 3)
                    | (5, 4, 0, 1)
                    | (5, 4, 1, 0)
                    | (5, 4, 2, 2)
                    | (5, 4, 3, 3) => [1, 0, 2, 3, 5, 4],
                    (0, 1, 2, 3)
                    | (0, 1, 3, 2)
                    | (0, 1, 4, 4)
                    | (0, 1, 5, 5)
                    | (1, 0, 2, 3)
                    | (1, 0, 3, 2)
                    | (1, 0, 4, 4)
                    | (1, 0, 5, 5)
                    | (2, 3, 0, 1)
                    | (2, 3, 1, 0)
                    | (2, 3, 4, 4)
                    | (2, 3, 5, 5)
                    | (3, 2, 0, 1)
                    | (3, 2, 1, 0)
                    | (3, 2, 4, 4)
                    | (3, 2, 5, 5)
                    | (4, 4, 0, 1)
                    | (4, 4, 1, 0)
                    | (4, 4, 2, 3)
                    | (4, 4, 3, 2)
                    | (5, 5, 0, 1)
                    | (5, 5, 1, 0)
                    | (5, 5, 2, 3)
                    | (5, 5, 3, 2) => [1, 0, 3, 2, 4, 5],
                    (0, 1, 2, 4)
                    | (0, 1, 3, 5)
                    | (0, 1, 4, 2)
                    | (0, 1, 5, 3)
                    | (1, 0, 2, 4)
                    | (1, 0, 3, 5)
                    | (1, 0, 4, 2)
                    | (1, 0, 5, 3)
                    | (2, 4, 0, 1)
                    | (2, 4, 1, 0)
                    | (2, 4, 4, 2)
                    | (2, 4, 5, 3)
                    | (3, 5, 0, 1)
                    | (3, 5, 1, 0)
                    | (3, 5, 4, 2)
                    | (3, 5, 5, 3)
                    | (4, 2, 0, 1)
                    | (4, 2, 1, 0)
                    | (4, 2, 2, 4)
                    | (4, 2, 3, 5)
                    | (5, 3, 0, 1)
                    | (5, 3, 1, 0)
                    | (5, 3, 2, 4)
                    | (5, 3, 3, 5) => [1, 0, 4, 5, 2, 3],
                    (0, 1, 2, 5)
                    | (0, 1, 3, 4)
                    | (0, 1, 4, 3)
                    | (0, 1, 5, 2)
                    | (1, 0, 2, 5)
                    | (1, 0, 3, 4)
                    | (1, 0, 4, 3)
                    | (1, 0, 5, 2)
                    | (2, 5, 0, 1)
                    | (2, 5, 1, 0)
                    | (2, 5, 4, 3)
                    | (2, 5, 5, 2)
                    | (3, 4, 0, 1)
                    | (3, 4, 1, 0)
                    | (3, 4, 4, 3)
                    | (3, 4, 5, 2)
                    | (4, 3, 0, 1)
                    | (4, 3, 1, 0)
                    | (4, 3, 2, 5)
                    | (4, 3, 3, 4)
                    | (5, 2, 0, 1)
                    | (5, 2, 1, 0)
                    | (5, 2, 2, 5)
                    | (5, 2, 3, 4) => [1, 0, 5, 4, 3, 2],
                    (0, 2, 1, 3)
                    | (0, 2, 2, 1)
                    | (0, 2, 3, 0)
                    | (0, 2, 4, 4)
                    | (0, 2, 5, 5)
                    | (1, 3, 0, 2)
                    | (1, 3, 2, 1)
                    | (1, 3, 3, 0)
                    | (1, 3, 4, 4)
                    | (1, 3, 5, 5)
                    | (2, 1, 0, 2)
                    | (2, 1, 1, 3)
                    | (2, 1, 3, 0)
                    | (2, 1, 4, 4)
                    | (2, 1, 5, 5)
                    | (3, 0, 0, 2)
                    | (3, 0, 1, 3)
                    | (3, 0, 2, 1)
                    | (3, 0, 4, 4)
                    | (3, 0, 5, 5)
                    | (4, 4, 0, 2)
                    | (4, 4, 1, 3)
                    | (4, 4, 2, 1)
                    | (4, 4, 3, 0)
                    | (5, 5, 0, 2)
                    | (5, 5, 1, 3)
                    | (5, 5, 2, 1)
                    | (5, 5, 3, 0) => [2, 3, 1, 0, 4, 5],
                    (0, 2, 2, 0)
                    | (0, 2, 3, 1)
                    | (0, 2, 4, 5)
                    | (0, 2, 5, 4)
                    | (1, 3, 2, 0)
                    | (1, 3, 3, 1)
                    | (1, 3, 4, 5)
                    | (1, 3, 5, 4)
                    | (2, 0, 0, 2)
                    | (2, 0, 1, 3)
                    | (2, 0, 4, 5)
                    | (2, 0, 5, 4)
                    | (3, 1, 0, 2)
                    | (3, 1, 1, 3)
                    | (3, 1, 4, 5)
                    | (3, 1, 5, 4)
                    | (4, 5, 0, 2)
                    | (4, 5, 1, 3)
                    | (4, 5, 2, 0)
                    | (4, 5, 3, 1)
                    | (5, 4, 0, 2)
                    | (5, 4, 1, 3)
                    | (5, 4, 2, 0)
                    | (5, 4, 3, 1) => [2, 3, 0, 1, 5, 4],
                    (0, 2, 2, 4)
                    | (0, 2, 3, 5)
                    | (0, 2, 4, 0)
                    | (0, 2, 5, 1)
                    | (1, 3, 2, 4)
                    | (1, 3, 3, 5)
                    | (1, 3, 4, 0)
                    | (1, 3, 5, 1)
                    | (2, 4, 0, 2)
                    | (2, 4, 1, 3)
                    | (2, 4, 4, 0)
                    | (2, 4, 5, 1)
                    | (3, 5, 0, 2)
                    | (3, 5, 1, 3)
                    | (3, 5, 4, 0)
                    | (3, 5, 5, 1)
                    | (4, 0, 0, 2)
                    | (4, 0, 1, 3)
                    | (4, 0, 2, 4)
                    | (4, 0, 3, 5)
                    | (5, 1, 0, 2)
                    | (5, 1, 1, 3)
                    | (5, 1, 2, 4)
                    | (5, 1, 3, 5) => [2, 3, 4, 5, 0, 1],
                    (0, 2, 2, 5)
                    | (0, 2, 3, 4)
                    | (0, 2, 4, 1)
                    | (0, 2, 5, 0)
                    | (1, 3, 2, 5)
                    | (1, 3, 3, 4)
                    | (1, 3, 4, 1)
                    | (1, 3, 5, 0)
                    | (2, 5, 0, 2)
                    | (2, 5, 1, 3)
                    | (2, 5, 4, 1)
                    | (2, 5, 5, 0)
                    | (3, 4, 0, 2)
                    | (3, 4, 1, 3)
                    | (3, 4, 4, 1)
                    | (3, 4, 5, 0)
                    | (4, 1, 0, 2)
                    | (4, 1, 1, 3)
                    | (4, 1, 2, 5)
                    | (4, 1, 3, 4)
                    | (5, 0, 0, 2)
                    | (5, 0, 1, 3)
                    | (5, 0, 2, 5)
                    | (5, 0, 3, 4) => [2, 3, 5, 4, 1, 0],
                    (0, 3, 1, 2)
                    | (0, 3, 2, 0)
                    | (0, 3, 3, 1)
                    | (0, 3, 4, 4)
                    | (0, 3, 5, 5)
                    | (1, 2, 0, 3)
                    | (1, 2, 2, 0)
                    | (1, 2, 3, 1)
                    | (1, 2, 4, 4)
                    | (1, 2, 5, 5)
                    | (2, 0, 0, 3)
                    | (2, 0, 1, 2)
                    | (2, 0, 3, 1)
                    | (2, 0, 4, 4)
                    | (2, 0, 5, 5)
                    | (3, 1, 0, 3)
                    | (3, 1, 1, 2)
                    | (3, 1, 2, 0)
                    | (3, 1, 4, 4)
                    | (3, 1, 5, 5)
                    | (4, 4, 0, 3)
                    | (4, 4, 1, 2)
                    | (4, 4, 2, 0)
                    | (4, 4, 3, 1)
                    | (5, 5, 0, 3)
                    | (5, 5, 1, 2)
                    | (5, 5, 2, 0)
                    | (5, 5, 3, 1) => [3, 2, 0, 1, 4, 5],
                    (0, 3, 2, 1)
                    | (0, 3, 3, 0)
                    | (0, 3, 4, 5)
                    | (0, 3, 5, 4)
                    | (1, 2, 2, 1)
                    | (1, 2, 3, 0)
                    | (1, 2, 4, 5)
                    | (1, 2, 5, 4)
                    | (2, 1, 0, 3)
                    | (2, 1, 1, 2)
                    | (2, 1, 4, 5)
                    | (2, 1, 5, 4)
                    | (3, 0, 0, 3)
                    | (3, 0, 1, 2)
                    | (3, 0, 4, 5)
                    | (3, 0, 5, 4)
                    | (4, 5, 0, 3)
                    | (4, 5, 1, 2)
                    | (4, 5, 2, 1)
                    | (4, 5, 3, 0)
                    | (5, 4, 0, 3)
                    | (5, 4, 1, 2)
                    | (5, 4, 2, 1)
                    | (5, 4, 3, 0) => [3, 2, 1, 0, 5, 4],
                    (0, 3, 2, 4)
                    | (0, 3, 3, 5)
                    | (0, 3, 4, 1)
                    | (0, 3, 5, 0)
                    | (1, 2, 2, 4)
                    | (1, 2, 3, 5)
                    | (1, 2, 4, 1)
                    | (1, 2, 5, 0)
                    | (2, 4, 0, 3)
                    | (2, 4, 1, 2)
                    | (2, 4, 4, 1)
                    | (2, 4, 5, 0)
                    | (3, 5, 0, 3)
                    | (3, 5, 1, 2)
                    | (3, 5, 4, 1)
                    | (3, 5, 5, 0)
                    | (4, 1, 0, 3)
                    | (4, 1, 1, 2)
                    | (4, 1, 2, 4)
                    | (4, 1, 3, 5)
                    | (5, 0, 0, 3)
                    | (5, 0, 1, 2)
                    | (5, 0, 2, 4)
                    | (5, 0, 3, 5) => [3, 2, 4, 5, 1, 0],
                    (0, 3, 2, 5)
                    | (0, 3, 3, 4)
                    | (0, 3, 4, 0)
                    | (0, 3, 5, 1)
                    | (1, 2, 2, 5)
                    | (1, 2, 3, 4)
                    | (1, 2, 4, 0)
                    | (1, 2, 5, 1)
                    | (2, 5, 0, 3)
                    | (2, 5, 1, 2)
                    | (2, 5, 4, 0)
                    | (2, 5, 5, 1)
                    | (3, 4, 0, 3)
                    | (3, 4, 1, 2)
                    | (3, 4, 4, 0)
                    | (3, 4, 5, 1)
                    | (4, 0, 0, 3)
                    | (4, 0, 1, 2)
                    | (4, 0, 2, 5)
                    | (4, 0, 3, 4)
                    | (5, 1, 0, 3)
                    | (5, 1, 1, 2)
                    | (5, 1, 2, 5)
                    | (5, 1, 3, 4) => [3, 2, 5, 4, 0, 1],
                    (0, 4, 1, 5)
                    | (0, 4, 2, 2)
                    | (0, 4, 3, 3)
                    | (0, 4, 4, 1)
                    | (0, 4, 5, 0)
                    | (1, 5, 0, 4)
                    | (1, 5, 2, 2)
                    | (1, 5, 3, 3)
                    | (1, 5, 4, 1)
                    | (1, 5, 5, 0)
                    | (2, 2, 0, 4)
                    | (2, 2, 1, 5)
                    | (2, 2, 4, 1)
                    | (2, 2, 5, 0)
                    | (3, 3, 0, 4)
                    | (3, 3, 1, 5)
                    | (3, 3, 4, 1)
                    | (3, 3, 5, 0)
                    | (4, 1, 0, 4)
                    | (4, 1, 1, 5)
                    | (4, 1, 2, 2)
                    | (4, 1, 3, 3)
                    | (4, 1, 5, 0)
                    | (5, 0, 0, 4)
                    | (5, 0, 1, 5)
                    | (5, 0, 2, 2)
                    | (5, 0, 3, 3)
                    | (5, 0, 4, 1) => [4, 5, 2, 3, 1, 0],
                    (0, 4, 2, 0)
                    | (0, 4, 3, 1)
                    | (0, 4, 4, 2)
                    | (0, 4, 5, 3)
                    | (1, 5, 2, 0)
                    | (1, 5, 3, 1)
                    | (1, 5, 4, 2)
                    | (1, 5, 5, 3)
                    | (2, 0, 0, 4)
                    | (2, 0, 1, 5)
                    | (2, 0, 4, 2)
                    | (2, 0, 5, 3)
                    | (3, 1, 0, 4)
                    | (3, 1, 1, 5)
                    | (3, 1, 4, 2)
                    | (3, 1, 5, 3)
                    | (4, 2, 0, 4)
                    | (4, 2, 1, 5)
                    | (4, 2, 2, 0)
                    | (4, 2, 3, 1)
                    | (5, 3, 0, 4)
                    | (5, 3, 1, 5)
                    | (5, 3, 2, 0)
                    | (5, 3, 3, 1) => [4, 5, 0, 1, 2, 3],
                    (0, 4, 2, 1)
                    | (0, 4, 3, 0)
                    | (0, 4, 4, 3)
                    | (0, 4, 5, 2)
                    | (1, 5, 2, 1)
                    | (1, 5, 3, 0)
                    | (1, 5, 4, 3)
                    | (1, 5, 5, 2)
                    | (2, 1, 0, 4)
                    | (2, 1, 1, 5)
                    | (2, 1, 4, 3)
                    | (2, 1, 5, 2)
                    | (3, 0, 0, 4)
                    | (3, 0, 1, 5)
                    | (3, 0, 4, 3)
                    | (3, 0, 5, 2)
                    | (4, 3, 0, 4)
                    | (4, 3, 1, 5)
                    | (4, 3, 2, 1)
                    | (4, 3, 3, 0)
                    | (5, 2, 0, 4)
                    | (5, 2, 1, 5)
                    | (5, 2, 2, 1)
                    | (5, 2, 3, 0) => [4, 5, 1, 0, 3, 2],
                    (0, 4, 2, 3)
                    | (0, 4, 3, 2)
                    | (0, 4, 4, 0)
                    | (0, 4, 5, 1)
                    | (1, 5, 2, 3)
                    | (1, 5, 3, 2)
                    | (1, 5, 4, 0)
                    | (1, 5, 5, 1)
                    | (2, 3, 0, 4)
                    | (2, 3, 1, 5)
                    | (2, 3, 4, 0)
                    | (2, 3, 5, 1)
                    | (3, 2, 0, 4)
                    | (3, 2, 1, 5)
                    | (3, 2, 4, 0)
                    | (3, 2, 5, 1)
                    | (4, 0, 0, 4)
                    | (4, 0, 1, 5)
                    | (4, 0, 2, 3)
                    | (4, 0, 3, 2)
                    | (5, 1, 0, 4)
                    | (5, 1, 1, 5)
                    | (5, 1, 2, 3)
                    | (5, 1, 3, 2) => [4, 5, 3, 2, 0, 1],
                    (0, 5, 1, 4)
                    | (0, 5, 2, 2)
                    | (0, 5, 3, 3)
                    | (0, 5, 4, 0)
                    | (0, 5, 5, 1)
                    | (1, 4, 0, 5)
                    | (1, 4, 2, 2)
                    | (1, 4, 3, 3)
                    | (1, 4, 4, 0)
                    | (1, 4, 5, 1)
                    | (2, 2, 0, 5)
                    | (2, 2, 1, 4)
                    | (2, 2, 4, 0)
                    | (2, 2, 5, 1)
                    | (3, 3, 0, 5)
                    | (3, 3, 1, 4)
                    | (3, 3, 4, 0)
                    | (3, 3, 5, 1)
                    | (4, 0, 0, 5)
                    | (4, 0, 1, 4)
                    | (4, 0, 2, 2)
                    | (4, 0, 3, 3)
                    | (4, 0, 5, 1)
                    | (5, 1, 0, 5)
                    | (5, 1, 1, 4)
                    | (5, 1, 2, 2)
                    | (5, 1, 3, 3)
                    | (5, 1, 4, 0) => [5, 4, 2, 3, 0, 1],
                    (0, 5, 2, 0)
                    | (0, 5, 3, 1)
                    | (0, 5, 4, 3)
                    | (0, 5, 5, 2)
                    | (1, 4, 2, 0)
                    | (1, 4, 3, 1)
                    | (1, 4, 4, 3)
                    | (1, 4, 5, 2)
                    | (2, 0, 0, 5)
                    | (2, 0, 1, 4)
                    | (2, 0, 4, 3)
                    | (2, 0, 5, 2)
                    | (3, 1, 0, 5)
                    | (3, 1, 1, 4)
                    | (3, 1, 4, 3)
                    | (3, 1, 5, 2)
                    | (4, 3, 0, 5)
                    | (4, 3, 1, 4)
                    | (4, 3, 2, 0)
                    | (4, 3, 3, 1)
                    | (5, 2, 0, 5)
                    | (5, 2, 1, 4)
                    | (5, 2, 2, 0)
                    | (5, 2, 3, 1) => [5, 4, 0, 1, 3, 2],
                    (0, 5, 2, 1)
                    | (0, 5, 3, 0)
                    | (0, 5, 4, 2)
                    | (0, 5, 5, 3)
                    | (1, 4, 2, 1)
                    | (1, 4, 3, 0)
                    | (1, 4, 4, 2)
                    | (1, 4, 5, 3)
                    | (2, 1, 0, 5)
                    | (2, 1, 1, 4)
                    | (2, 1, 4, 2)
                    | (2, 1, 5, 3)
                    | (3, 0, 0, 5)
                    | (3, 0, 1, 4)
                    | (3, 0, 4, 2)
                    | (3, 0, 5, 3)
                    | (4, 2, 0, 5)
                    | (4, 2, 1, 4)
                    | (4, 2, 2, 1)
                    | (4, 2, 3, 0)
                    | (5, 3, 0, 5)
                    | (5, 3, 1, 4)
                    | (5, 3, 2, 1)
                    | (5, 3, 3, 0) => [5, 4, 1, 0, 2, 3],
                    (0, 5, 2, 3)
                    | (0, 5, 3, 2)
                    | (0, 5, 4, 1)
                    | (0, 5, 5, 0)
                    | (1, 4, 2, 3)
                    | (1, 4, 3, 2)
                    | (1, 4, 4, 1)
                    | (1, 4, 5, 0)
                    | (2, 3, 0, 5)
                    | (2, 3, 1, 4)
                    | (2, 3, 4, 1)
                    | (2, 3, 5, 0)
                    | (3, 2, 0, 5)
                    | (3, 2, 1, 4)
                    | (3, 2, 4, 1)
                    | (3, 2, 5, 0)
                    | (4, 1, 0, 5)
                    | (4, 1, 1, 4)
                    | (4, 1, 2, 3)
                    | (4, 1, 3, 2)
                    | (5, 0, 0, 5)
                    | (5, 0, 1, 4)
                    | (5, 0, 2, 3)
                    | (5, 0, 3, 2) => [5, 4, 3, 2, 1, 0],
                    x => panic!("{:?}", x),
                };
                AxisMap::Map2 { map }
            }
            m => m,
        }
    }

    pub fn map_axis(self, direction: u8, directions: &[u8]) -> Vec<u8> {
        match self {
            AxisMap::None => directions.to_vec(),
            AxisMap::Map1 { from, to } => {
                if from == direction || from == direction ^ 1 {
                    return vec![to];
                }
                let map = match (from, to) {
                    (from, to) if from == to => [0, 1, 2, 3, 4, 5],
                    (0, 1) | (1, 0) => [1, 0, 2, 3, 5, 4],
                    (0, 2) | (2, 0) => [2, 3, 0, 1, 5, 4],
                    (0, 3) | (3, 0) => [3, 2, 1, 0, 5, 4],
                    (0, 4) | (4, 0) => [4, 5, 3, 2, 0, 1],
                    (0, 5) | (5, 0) => [5, 4, 3, 2, 1, 0],
                    (1, 2) | (2, 1) => [3, 2, 1, 0, 5, 4],
                    (1, 3) | (3, 1) => [2, 3, 0, 1, 5, 4],
                    (1, 4) | (4, 1) => [5, 4, 3, 2, 1, 0],
                    (1, 5) | (5, 1) => [4, 5, 3, 2, 0, 1],
                    (2, 3) | (3, 2) => [0, 1, 3, 2, 5, 4],
                    (2, 4) | (4, 2) => [1, 0, 4, 5, 2, 3],
                    (2, 5) | (5, 2) => [1, 0, 5, 4, 3, 2],
                    (3, 4) | (4, 3) => [1, 0, 5, 4, 3, 2],
                    (3, 5) | (5, 3) => [1, 0, 4, 5, 2, 3],
                    (4, 5) | (5, 4) => [0, 1, 3, 2, 5, 4],
                    _ => unreachable!(),
                };
                directions
                    .iter()
                    .filter_map(|&d| {
                        if d == from || d == from ^ 1 {
                            None
                        } else {
                            Some(map[d as usize])
                        }
                    })
                    .collect()
            }
            AxisMap::Map2 { map } => {
                vec![map[direction as usize]]
            }
        }
    }
}

impl<T: Copy> Grid3<T> {
    pub fn new(d: u8, init: T) -> Grid3<T> {
        let size = d as usize;
        Grid3 {
            d,
            data: vec![init; size * size * size],
        }
    }
}

impl<T> Grid3<T> {
    fn at(&self, p: Point) -> usize {
        let Point(x, y, z) = p;
        let d = self.d as usize;
        let x = x as usize;
        let y = y as usize;
        let z = z as usize;
        (x * d + y) * d + z
    }
}

impl<T: Copy> GridFront<T> {
    pub fn new(d: u8, init: T) -> GridFront<T> {
        let size = d as usize;
        GridFront {
            d,
            data: vec![init; size * size],
        }
    }
}

impl<T: Copy> GridRight<T> {
    pub fn new(d: u8, init: T) -> GridRight<T> {
        let size = d as usize;
        GridRight {
            d,
            data: vec![init; size * size],
        }
    }
}

impl<T> GridFront<T> {
    pub fn from_vec(d: u8, data: Vec<T>) -> GridFront<T> {
        GridFront { d, data }
    }

    fn at(&self, p: Point) -> usize {
        let Point(x, _, z) = p;
        (x * self.d + z) as usize
    }
}

impl<T> GridRight<T> {
    pub fn from_vec(d: u8, data: Vec<T>) -> GridRight<T> {
        GridRight { d, data }
    }

    fn at(&self, p: Point) -> usize {
        let Point(_, y, z) = p;
        (z * self.d + y) as usize
    }

    pub fn row(&self, z: usize) -> &[T] {
        let d = self.d as usize;
        &self.data[z * d..(z + 1) * d]
    }
}

impl<T> Index<Point> for Grid3<T> {
    type Output = T;
    fn index(&self, p: Point) -> &T {
        &self.data[self.at(p)]
    }
}

impl<T> IndexMut<Point> for Grid3<T> {
    fn index_mut(&mut self, p: Point) -> &mut T {
        let i = self.at(p);
        &mut self.data[i]
    }
}

impl<T> Index<Point> for GridFront<T> {
    type Output = T;
    fn index(&self, p: Point) -> &T {
        &self.data[self.at(p)]
    }
}

impl<T> IndexMut<Point> for GridFront<T> {
    fn index_mut(&mut self, p: Point) -> &mut T {
        let i = self.at(p);
        &mut self.data[i]
    }
}

impl<T> Index<Point> for GridRight<T> {
    type Output = T;
    fn index(&self, p: Point) -> &T {
        &self.data[self.at(p)]
    }
}

impl<T> IndexMut<Point> for GridRight<T> {
    fn index_mut(&mut self, p: Point) -> &mut T {
        let i = self.at(p);
        &mut self.data[i]
    }
}
