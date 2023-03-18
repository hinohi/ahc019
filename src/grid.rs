use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Point(usize, usize, usize);

#[derive(Debug)]
pub struct Grid3<T> {
    d: usize,
    pub data: Vec<T>,
}

#[derive(Debug)]
pub struct GridFront<T> {
    d: usize,
    pub data: Vec<T>,
}

#[derive(Debug)]
pub struct GridRight<T> {
    d: usize,
    pub data: Vec<T>,
}

impl Point {
    pub const fn new(x: usize, y: usize, z: usize) -> Point {
        Point(x, y, z)
    }

    pub fn to_x(self, d: usize, dx: usize) -> Option<Point> {
        let x = self.0.wrapping_add(dx);
        if x < d {
            Some(Point(x, self.1, self.2))
        } else {
            None
        }
    }

    pub fn to_y(self, d: usize, dy: usize) -> Option<Point> {
        let y = self.1.wrapping_add(dy);
        if y < d {
            Some(Point(self.0, y, self.2))
        } else {
            None
        }
    }

    pub fn to_z(self, d: usize, dz: usize) -> Option<Point> {
        let z = self.2.wrapping_add(dz);
        if z < d {
            Some(Point(self.0, self.1, z))
        } else {
            None
        }
    }

    pub fn next_cell(self, d: usize, direction: u8) -> Option<Point> {
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

impl<T: Copy> Grid3<T> {
    pub fn new(d: usize, init: T) -> Grid3<T> {
        Grid3 {
            d,
            data: vec![init; d * d * d],
        }
    }
}

impl<T: Copy> GridFront<T> {
    pub fn new(d: usize, init: T) -> GridFront<T> {
        GridFront {
            d,
            data: vec![init; d * d],
        }
    }
}

impl<T: Copy> GridRight<T> {
    pub fn new(d: usize, init: T) -> GridRight<T> {
        GridRight {
            d,
            data: vec![init; d * d],
        }
    }
}

impl<T: Eq> GridFront<T> {
    pub fn any_eq(&self, v: T) -> bool {
        self.data.iter().any(|i| *i == v)
    }
}

impl<T: Eq> GridRight<T> {
    pub fn any_eq(&self, v: T) -> bool {
        self.data.iter().any(|i| *i == v)
    }
}

impl<T> GridFront<T> {
    pub fn from_vec(d: usize, data: Vec<T>) -> GridFront<T> {
        GridFront { d, data }
    }
}

impl<T> GridRight<T> {
    pub fn from_vec(d: usize, data: Vec<T>) -> GridRight<T> {
        GridRight { d, data }
    }
}

impl<T> Index<Point> for Grid3<T> {
    type Output = T;
    fn index(&self, index: Point) -> &T {
        let Point(x, y, z) = index;
        &self.data[(x * self.d + y) * self.d + z]
    }
}

impl<T> IndexMut<Point> for Grid3<T> {
    fn index_mut(&mut self, index: Point) -> &mut T {
        let Point(x, y, z) = index;
        &mut self.data[(x * self.d + y) * self.d + z]
    }
}

impl<T> Index<Point> for GridFront<T> {
    type Output = T;
    fn index(&self, index: Point) -> &T {
        let Point(x, _, z) = index;
        &self.data[x * self.d + z]
    }
}

impl<T> IndexMut<Point> for GridFront<T> {
    fn index_mut(&mut self, index: Point) -> &mut T {
        let Point(x, _, z) = index;
        &mut self.data[x * self.d + z]
    }
}

impl<T> Index<Point> for GridRight<T> {
    type Output = T;
    fn index(&self, index: Point) -> &T {
        let Point(_, y, z) = index;
        &self.data[y * self.d + z]
    }
}

impl<T> IndexMut<Point> for GridRight<T> {
    fn index_mut(&mut self, index: Point) -> &mut T {
        let Point(_, y, z) = index;
        &mut self.data[y * self.d + z]
    }
}
