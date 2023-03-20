use ahc019::{AxisMap, Grid3, GridFront, GridRight, McScheduler, Point};
use proconio::{input, marker::Bytes};
use rand::seq::SliceRandom;
use rand::Rng;
use rand_pcg::Mcg128Xsl64;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

#[derive(Clone)]
pub struct GridBox {
    d: u8,
    grid: Grid3<u16>,
    front: GridFront<u8>,
    right: GridRight<u8>,
}

pub struct YetPointSet {
    yet_yet: Vec<Point>,
    yet: Vec<Point>,
    can: Vec<Point>,
}

#[derive(Clone)]
pub struct Block {
    shared: Vec<(u16, Vec<Point>, Vec<Point>)>,
    shared_id_stock: Vec<u16>,
    half1: Vec<Point>,
    half2: Vec<Point>,
    next_half_id: u16,
}

pub fn make_face(shadow: &[Vec<u8>]) -> Vec<u8> {
    let d = shadow.len();
    let mut v = vec![!0; d * d];
    for (i, row) in shadow.iter().enumerate() {
        for (j, &f) in row.iter().enumerate() {
            if f == b'1' {
                v[j * d + i] = 0;
            }
        }
    }
    v
}

impl GridBox {
    pub fn new(d: u8, front: &[Vec<u8>], right: &[Vec<u8>]) -> GridBox {
        let mut grid = Grid3::new(d, 0);
        let front = GridFront::from_vec(d, make_face(&front));
        let right = GridRight::from_vec(d, make_face(&right));
        for x in 0..d {
            for y in 0..d {
                for z in 0..d {
                    let p = Point::new(x, y, z);
                    if front[p] == !0 || right[p] == !0 {
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
        let mut yet_yet = Vec::new();
        let mut yet = Vec::new();
        let mut can = Vec::new();
        for x in 0..self.d {
            for y in 0..self.d {
                for z in 0..self.d {
                    let p = Point::new(x, y, z);
                    match (self.front[p], self.right[p]) {
                        (0, 0) => yet_yet.push(p),
                        (0, x) | (x, 0) if x != !0 => {
                            if yet_yet.is_empty() {
                                yet.push(p)
                            }
                        }
                        (x, y) if x != !0 && y != !0 => {
                            if yet_yet.is_empty() && yet.is_empty() && self.grid[p] == 0 {
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
        self.front[p] += 1;
        self.right[p] += 1;
    }

    pub fn remove(&mut self, p: Point) {
        debug_assert_ne!(self.grid[p], 0);
        debug_assert!(self.front[p] > 0);
        debug_assert!(self.right[p] > 0);
        self.grid[p] = 0;
        self.front[p] -= 1;
        self.right[p] -= 1;
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

impl Block {
    pub fn new() -> Block {
        Block {
            shared: Vec::new(),
            shared_id_stock: Vec::new(),
            half1: Vec::new(),
            half2: Vec::new(),
            next_half_id: 10000,
        }
    }

    pub fn shared_only_score(&self) -> f64 {
        let mut score = 0.0;
        for (_, v, _) in self.shared.iter() {
            score += 1.0 / v.len() as f64;
        }
        score
    }

    pub fn gen_shared_block_id(&mut self) -> u16 {
        if let Some(id) = self.shared_id_stock.pop() {
            id
        } else {
            (self.shared.len() + 1) as u16
        }
    }

    pub fn gen_half_block_id(&mut self) -> u16 {
        let id = self.next_half_id;
        self.next_half_id += 1;
        id
    }

    pub fn push_shared(&mut self, block_id: u16, p1: Vec<Point>, p2: Vec<Point>) {
        self.shared.push((block_id, p1, p2));
    }

    fn pop_shared(&mut self, i: usize) -> (Vec<Point>, Vec<Point>) {
        let (id, s1, s2) = self.shared.swap_remove(i);
        self.shared_id_stock.push(id);
        (s1, s2)
    }

    pub fn pop_random(&mut self, rng: &mut Mcg128Xsl64) -> (Vec<Point>, Vec<Point>) {
        let i = rng.gen_range(0, self.shared.len());
        self.pop_shared(i)
    }

    pub fn pop_small(&mut self, th: usize) -> Option<(Vec<Point>, Vec<Point>)> {
        for i in 0..self.shared.len() {
            if self.shared[i].1.len() <= th {
                return Some(self.pop_shared(i));
            }
        }
        None
    }

    pub fn push_half(&mut self, place: u8, p: Point) {
        if place == 1 {
            self.half1.push(p);
        } else {
            self.half2.push(p);
        }
    }

    pub fn half_reset(&mut self) {
        self.half1.clear();
        self.half2.clear();
        self.next_half_id = 10000;
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

fn grow_shared_block(
    rng: &mut Mcg128Xsl64,
    grid_1: &mut GridBox,
    grid_2: &mut GridBox,
    block_id: u16,
    p1: Point,
    p2: Point,
) -> (Vec<Point>, Vec<Point>) {
    let d = grid_1.d;
    let mut directions = [0, 1, 2, 3, 4, 5];
    directions.shuffle(rng);
    let mut axis_map = AxisMap::new();
    let mut pp1 = Vec::new();
    let mut pp2 = Vec::new();
    let mut stack = vec![(p1, p2)];
    grid_1.put(p1, block_id);
    grid_2.put(p2, block_id);
    pp1.push(p1);
    pp2.push(p2);
    while let Some((p1, p2)) = stack.pop() {
        for &dir1 in directions.iter() {
            if let Some(p1) = p1.next_cell(d, dir1) {
                if grid_1.grid[p1] != 0 {
                    continue;
                }
                for dir2 in axis_map.map_axis(dir1, &directions) {
                    if let Some(p2) = p2.next_cell(d, dir2) {
                        if grid_2.grid[p2] == 0 {
                            grid_1.put(p1, block_id);
                            grid_2.put(p2, block_id);
                            pp1.push(p1);
                            pp2.push(p2);
                            axis_map = axis_map.fix(dir1, dir2);
                            stack.push((p1, p2));
                            break;
                        }
                    }
                }
            }
        }
    }
    (pp1, pp2)
}

fn fill_all(
    rng: &mut Mcg128Xsl64,
    grid_1: &mut GridBox,
    grid_2: &mut GridBox,
    block: &mut Block,
    cut_off: f64,
) -> Option<f64> {
    fn single_update_loop(
        rng: &mut Mcg128Xsl64,
        directions: &mut [u8],
        grid: &mut GridBox,
        mut p: Point,
        block: &mut Block,
        place: u8,
    ) -> f64 {
        let block_id = block.gen_half_block_id();
        let mut c = 0;
        loop {
            c += 1;
            directions.shuffle(rng);
            grid.put(p, block_id);
            block.push_half(place, p);
            if let Some(q) = chose_next1(&grid, p, &directions) {
                p = q;
            } else {
                break;
            }
        }
        1.0 / c as f64 + c as f64
    }

    let mut directions = [0, 1, 2, 3, 4, 5];
    let mut score = 0.0;
    loop {
        if score >= cut_off {
            return None;
        }
        let yet1 = grid_1.make_can_put_points();
        let yet2 = grid_2.make_can_put_points();
        if yet1.satisfied() && yet2.satisfied() {
            break;
        }
        let p1 = yet1.chose(rng);
        let p2 = yet2.chose(rng);
        match (p1, p2) {
            (Some(p1), Some(p2)) => {
                let block_id = block.gen_shared_block_id();
                let (pp1, pp2) = grow_shared_block(rng, grid_1, grid_2, block_id, p1, p2);
                score += 1.0 / pp1.len() as f64;
                block.push_shared(block_id, pp1, pp2);
            }
            (Some(p), None) => {
                score += single_update_loop(rng, &mut directions, grid_1, p, block, 1);
            }
            (None, Some(p)) => {
                score += single_update_loop(rng, &mut directions, grid_2, p, block, 2);
            }
            (None, None) => return None,
        }
    }
    Some(score)
}

fn solve(
    rng: &mut Mcg128Xsl64,
    d: u8,
    front1: &[Vec<u8>],
    right1: &[Vec<u8>],
    front2: &[Vec<u8>],
    right2: &[Vec<u8>],
    scheduler: McScheduler,
) -> (Vec<u16>, Vec<u16>, f64) {
    let mut grid_1 = GridBox::new(d, &front1, right1);
    let mut grid_2 = GridBox::new(d, &front2, right2);
    let mut block = Block::new();
    let mut best = loop {
        if let Some(score) = fill_all(rng, &mut grid_1, &mut grid_2, &mut block, 1e100) {
            break (grid_1.grid.data.clone(), grid_2.grid.data.clone(), score);
        }
        grid_1 = GridBox::new(d, &front1, right1);
        grid_2 = GridBox::new(d, &front2, right2);
        block = Block::new();
    };
    let mut score = best.2;
    for step in 0..scheduler.max_step {
        let temperature = scheduler.temperature(step);

        let before_state = (grid_1.clone(), grid_2.clone(), block.clone());

        for &p in block.half1.iter() {
            grid_1.remove(p);
        }
        for &p in block.half2.iter() {
            grid_2.remove(p);
        }
        block.half_reset();
        while let Some((p1, p2)) = block.pop_small(2) {
            for p in p1 {
                grid_1.remove(p);
            }
            for p in p2 {
                grid_2.remove(p);
            }
        }
        if !block.shared.is_empty() {
            let (p1, p2) = block.pop_random(rng);
            for p in p1 {
                grid_1.remove(p);
            }
            for p in p2 {
                grid_2.remove(p);
            }
        }

        let sos = block.shared_only_score();
        let cut_off = temperature * 10.0 - sos + score;
        let new_score =
            sos + fill_all(rng, &mut grid_1, &mut grid_2, &mut block, cut_off).unwrap_or(1e100);
        if new_score > score || !rng.gen_bool(((new_score - score) / temperature).exp()) {
            grid_1 = before_state.0;
            grid_2 = before_state.1;
            block = before_state.2;
        } else {
            score = new_score;
            if score < best.2 {
                best = (grid_1.grid.data.clone(), grid_2.grid.data.clone(), score);
            }
        }
    }
    best
}

fn print_ans(v: &[u16], block_id_map: &HashMap<u16, usize>) {
    for (i, g) in v.iter().enumerate() {
        if i != 0 {
            print!(" ");
        }
        print!("{}", block_id_map.get(g).unwrap_or(&0));
    }
    println!();
}

fn main() {
    let start = Instant::now();
    input! {
        d: u8,
        front1: [Bytes; d],
        right1: [Bytes; d],
        front2: [Bytes; d],
        right2: [Bytes; d],
    }
    let scheduler = McScheduler::new(1000, 20.0, 1e-4);
    let mut rng = Mcg128Xsl64::new(9085);
    let mut best_score = 1e300;
    let mut best = (Vec::new(), Vec::new());
    let mut mc_run = 0;
    while start.elapsed() < Duration::from_millis(5500) {
        mc_run += 1;
        let (g1, g2, score) = solve(&mut rng, d, &front1, &right1, &front2, &right2, scheduler);
        if score < best_score {
            best_score = score;
            best = (g1, g2);
        }
    }
    let (g1, g2) = best;
    let mut block_id_map = HashMap::new();
    for &g in g1.iter().chain(g2.iter()) {
        if g == 0 || g == !0 {
            continue;
        }
        let id = block_id_map.len() + 1;
        block_id_map.entry(g).or_insert(id);
    }
    println!("{}", block_id_map.len());
    print_ans(&g1, &block_id_map);
    print_ans(&g2, &block_id_map);
    eprintln!("{} {}", mc_run, best_score);
}
