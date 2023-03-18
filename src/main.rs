use ahc019::{Grid3, GridFront, GridRight, Point};
use proconio::{input, marker::Bytes};
use rand::seq::SliceRandom;
use rand_pcg::Mcg128Xsl64;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FaceState {
    Null,
    Yet,
    Satisfied,
}

pub fn make_face(shadow: &[Vec<u8>]) -> Vec<FaceState> {
    let d = shadow.len();
    let mut v = vec![FaceState::Null; d * d];
    for (i, row) in shadow.iter().enumerate() {
        for (j, &f) in row.iter().enumerate() {
            if f == b'1' {
                v[j * d + i] = FaceState::Yet;
            }
        }
    }
    v
}

pub fn make_can_put_points(
    d: usize,
    grid: &Grid3<u16>,
    front: &GridFront<FaceState>,
    right: &GridRight<FaceState>,
) -> YetPointSet {
    use FaceState::*;
    let mut yet_yet = Vec::new();
    let mut yet = Vec::new();
    let mut can = Vec::new();
    for x in 0..d {
        for y in 0..d {
            for z in 0..d {
                let p = Point::new(x, y, z);
                match (front[p], right[p]) {
                    (Yet, Yet) => yet_yet.push(p),
                    (Yet, Satisfied) | (Satisfied, Yet) => yet.push(p),
                    (Satisfied, Satisfied) => {
                        if grid[p] == 0 {
                            can.push(p)
                        }
                    }
                    _ => (),
                }
            }
        }
    }
    YetPointSet { yet_yet, yet, can }
}

pub struct YetPointSet {
    yet_yet: Vec<Point>,
    yet: Vec<Point>,
    can: Vec<Point>,
}

impl YetPointSet {
    pub fn satisfied(&self) -> bool {
        self.yet_yet.is_empty() && self.yet.is_empty()
    }

    pub fn chose(&self, rng: &mut Mcg128Xsl64) -> Option<Point> {
        if !self.yet_yet.is_empty() {
            Some(*self.yet_yet.choose(rng).unwrap())
        } else if !self.yet.is_empty() {
            Some(*self.yet.choose(rng).unwrap())
        } else if !self.can.is_empty() {
            Some(*self.can.choose(rng).unwrap())
        } else {
            None
        }
    }
}

fn chose_next1(d: usize, grid: &Grid3<u16>, p: Point, directions: &[u8]) -> Option<Point> {
    for &dir in directions.iter() {
        if let Some(q) = p.next_cell(d, dir) {
            if grid[q] == 0 {
                return Some(q);
            }
        }
    }
    None
}

fn chose_next2(
    d: usize,
    grid_1: &Grid3<u16>,
    grid_2: &Grid3<u16>,
    p1: Point,
    p2: Point,
    directions: &[u8],
) -> Option<(Point, Point)> {
    for &dir in directions.iter() {
        if let Some(q1) = p1.next_cell(d, dir) {
            if let Some(q2) = p2.next_cell(d, dir) {
                if grid_1[q1] == 0 && grid_2[q2] == 0 {
                    return Some((q1, q2));
                }
            }
        }
    }
    None
}

fn solve(
    rng: &mut Mcg128Xsl64,
    d: usize,
    front1: &[Vec<u8>],
    right1: &[Vec<u8>],
    front2: &[Vec<u8>],
    right2: &[Vec<u8>],
) -> Option<(u16, Vec<u16>, Vec<u16>, u64)> {
    let mut grid_1 = Grid3::new(d, 0u16);
    let mut grid_2 = Grid3::new(d, 0u16);
    let mut grid_f1 = GridFront::from_vec(d, make_face(&front1));
    let mut grid_r1 = GridRight::from_vec(d, make_face(&right1));
    let mut grid_f2 = GridFront::from_vec(d, make_face(&front2));
    let mut grid_r2 = GridRight::from_vec(d, make_face(&right2));
    for x in 0..d {
        for y in 0..d {
            for z in 0..d {
                let p = Point::new(x, y, z);
                if grid_f1[p] == FaceState::Null || grid_r1[p] == FaceState::Null {
                    grid_1[p] = !0;
                }
                if grid_f2[p] == FaceState::Null || grid_r2[p] == FaceState::Null {
                    grid_2[p] = !0;
                }
            }
        }
    }
    let mut block_id = 1;
    let mut directions = [0, 1, 2, 3, 4, 5];
    loop {
        let yet1 = make_can_put_points(d, &grid_1, &grid_f1, &grid_r1);
        let yet2 = make_can_put_points(d, &grid_2, &grid_f2, &grid_r2);
        if yet1.satisfied() && yet2.satisfied() {
            break;
        }
        let p1 = yet1.chose(rng);
        let p2 = yet2.chose(rng);
        match (p1, p2) {
            (Some(mut p1), Some(mut p2)) => loop {
                directions.shuffle(rng);
                grid_1[p1] = block_id;
                grid_2[p2] = block_id;
                grid_f1[p1] = FaceState::Satisfied;
                grid_r1[p1] = FaceState::Satisfied;
                grid_f2[p2] = FaceState::Satisfied;
                grid_r2[p2] = FaceState::Satisfied;
                if let Some((q1, q2)) = chose_next2(d, &grid_1, &grid_2, p1, p2, &directions) {
                    p1 = q1;
                    p2 = q2;
                } else {
                    break;
                }
            },
            (Some(mut p), None) => loop {
                directions.shuffle(rng);
                grid_1[p] = block_id;
                grid_f1[p] = FaceState::Satisfied;
                grid_r1[p] = FaceState::Satisfied;
                if let Some(q) = chose_next1(d, &grid_1, p, &directions) {
                    p = q;
                } else {
                    break;
                }
            },
            (None, Some(mut p)) => loop {
                directions.shuffle(rng);
                grid_2[p] = block_id;
                grid_f2[p] = FaceState::Satisfied;
                grid_r2[p] = FaceState::Satisfied;
                if let Some(q) = chose_next1(d, &grid_2, p, &directions) {
                    p = q;
                } else {
                    break;
                }
            },
            (None, None) => return None,
        }
        block_id += 1;
    }
    Some((block_id - 1, grid_1.data, grid_2.data, 100))
}

fn main() {
    input! {
        d: usize,
        front1: [Bytes; d],
        right1: [Bytes; d],
        front2: [Bytes; d],
        right2: [Bytes; d],
    }
    let mut rng = Mcg128Xsl64::new(9085);
    loop {
        if let Some((n, g1, g2, score)) = solve(&mut rng, d, &front1, &right1, &front2, &right2) {
            println!("{}", n);
            for (i, &g) in g1.iter().enumerate() {
                if i != 0 {
                    print!(" ");
                }
                print!("{}", if g == !0 { 0 } else { g });
            }
            println!();
            for (i, &g) in g2.iter().enumerate() {
                if i != 0 {
                    print!(" ");
                }
                print!("{}", if g == !0 { 0 } else { g });
            }
            println!();
            eprintln!("{}", score);
        }
        break;
    }
}
