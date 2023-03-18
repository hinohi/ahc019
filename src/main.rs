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

pub struct GridBox {
    d: usize,
    grid: Grid3<u16>,
    front: GridFront<FaceState>,
    right: GridRight<FaceState>,
}

pub struct YetPointSet {
    yet_yet: Vec<Point>,
    yet: Vec<Point>,
    can: Vec<Point>,
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

impl GridBox {
    pub fn new(d: usize, front: &[Vec<u8>], right: &[Vec<u8>]) -> GridBox {
        let mut grid = Grid3::new(d, 0);
        let front = GridFront::from_vec(d, make_face(&front));
        let right = GridRight::from_vec(d, make_face(&right));
        for x in 0..d {
            for y in 0..d {
                for z in 0..d {
                    let p = Point::new(x, y, z);
                    if front[p] == FaceState::Null || right[p] == FaceState::Null {
                        grid[p] = !0;
                    }
                }
            }
        }
        GridBox {
            d,
            grid,
            front,
            right,
        }
    }

    pub fn make_can_put_points(&self) -> YetPointSet {
        use FaceState::*;
        let mut yet_yet = Vec::new();
        let mut yet = Vec::new();
        let mut can = Vec::new();
        for x in 0..self.d {
            for y in 0..self.d {
                for z in 0..self.d {
                    let p = Point::new(x, y, z);
                    match (self.front[p], self.right[p]) {
                        (Yet, Yet) => yet_yet.push(p),
                        (Yet, Satisfied) | (Satisfied, Yet) => yet.push(p),
                        (Satisfied, Satisfied) => {
                            if self.grid[p] == 0 {
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

    pub fn put(&mut self, p: Point, block_id: u16) {
        self.grid[p] = block_id;
        self.front[p] = FaceState::Satisfied;
        self.right[p] = FaceState::Satisfied;
    }
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

pub fn chose_next1(grid: &GridBox, p: Point, directions: &[u8]) -> Option<Point> {
    for &dir in directions.iter() {
        if let Some(q) = p.next_cell(grid.d, dir) {
            if grid.grid[q] == 0 {
                return Some(q);
            }
        }
    }
    None
}

pub fn chose_next2(
    grid_1: &GridBox,
    grid_2: &GridBox,
    p1: Point,
    p2: Point,
    directions: &[u8],
) -> Option<(Point, Point)> {
    let d = grid_1.d;
    for &dir in directions.iter() {
        if let Some(q1) = p1.next_cell(d, dir) {
            if let Some(q2) = p2.next_cell(d, dir) {
                if grid_1.grid[q1] == 0 && grid_2.grid[q2] == 0 {
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
) -> Option<(u16, Vec<u16>, Vec<u16>, f64)> {
    fn single_update_loop(
        rng: &mut Mcg128Xsl64,
        directions: &mut [u8],
        grid: &mut GridBox,
        mut p: Point,
        block_id: u16,
    ) -> f64 {
        let mut c = 0;
        loop {
            c += 1;
            directions.shuffle(rng);
            grid.put(p, block_id);
            if let Some(q) = chose_next1(&grid, p, &directions) {
                p = q;
            } else {
                break;
            }
        }
        1.0 / c as f64 + c as f64
    }

    let mut grid_1 = GridBox::new(d, &front1, right1);
    let mut grid_2 = GridBox::new(d, &front2, right2);
    let mut block_id = 0;
    let mut directions = [0, 1, 2, 3, 4, 5];
    let mut score = 0.0;

    loop {
        let yet1 = grid_1.make_can_put_points();
        let yet2 = grid_2.make_can_put_points();
        if yet1.satisfied() && yet2.satisfied() {
            break;
        }
        block_id += 1;
        let p1 = yet1.chose(rng);
        let p2 = yet2.chose(rng);
        match (p1, p2) {
            (Some(mut p1), Some(mut p2)) => {
                let mut c = 0;
                loop {
                    c += 1;
                    directions.shuffle(rng);
                    grid_1.put(p1, block_id);
                    grid_2.put(p2, block_id);
                    if let Some((q1, q2)) = chose_next2(&grid_1, &grid_2, p1, p2, &directions) {
                        p1 = q1;
                        p2 = q2;
                    } else {
                        break;
                    }
                }
                score += 1.0 / c as f64;
            }
            (Some(p), None) => {
                score += single_update_loop(rng, &mut directions, &mut grid_1, p, block_id);
            }
            (None, Some(p)) => {
                score += single_update_loop(rng, &mut directions, &mut grid_2, p, block_id);
            }
            (None, None) => return None,
        }
    }
    Some((block_id, grid_1.grid.data, grid_2.grid.data, score))
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
    let mut best_score = std::f64::MAX;
    let mut best = (0, Vec::new(), Vec::new());
    for _ in 0..1000 {
        if let Some((n, g1, g2, score)) = solve(&mut rng, d, &front1, &right1, &front2, &right2) {
            if score < best_score {
                best_score = score;
                best = (n, g1, g2);
            }
        }
    }
    let (n, g1, g2) = best;
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
    eprintln!("{}", best_score);
}
