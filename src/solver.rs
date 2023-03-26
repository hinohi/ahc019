use crate::{AxisMap, BlockSet, Grid3, GridFront, GridRight, McParams, Point};
use rand::{seq::SliceRandom, Rng};
use rand_pcg::Mcg128Xsl64;
use smallvec::{smallvec, SmallVec};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GridBox {
    d: u8,
    grid: Grid3<u16>,
    front: GridFront<u8>,
    right: GridRight<u8>,
}

pub struct YetPointSet {
    yet_yet: SmallVec<[Point; 128]>,
    yet: SmallVec<[Point; 16]>,
    can: SmallVec<[Point; 16]>,
}

type Hole = (u8, u8, Vec<u8>);

pub fn make_face(shadow: &[Vec<u8>], t: bool) -> Vec<u8> {
    let d = shadow.len();
    let mut v = vec![!0; d * d];
    for (i, row) in shadow.iter().enumerate() {
        for (j, &f) in row.iter().enumerate() {
            if f == b'1' {
                if t {
                    v[i * d + j] = 0;
                } else {
                    v[j * d + i] = 0;
                }
            }
        }
    }
    v
}

impl GridBox {
    pub fn new(d: u8, front: &[Vec<u8>], right: &[Vec<u8>]) -> GridBox {
        let mut grid = Grid3::new(d, 0);
        let front = GridFront::from_vec(d, make_face(front, false));
        let right = GridRight::from_vec(d, make_face(right, true));
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

    pub fn reset(&mut self, hole: &[Hole]) {
        for (x, z, yy) in hole.iter() {
            let x = *x;
            let z = *z;
            self.front[(x, z)] = 0;
            for &y in yy.iter() {
                self.right[(y, z)] = 0;
                let p = Point::new(x, y, z);
                self.grid[p] = 0;
            }
        }
    }

    pub fn make_hole_xzy(&self) -> Vec<Hole> {
        let mut v = Vec::new();
        for x in 0..self.d {
            for z in 0..self.d {
                let front = self.front[(x, z)];
                if front == !0 {
                    continue;
                }
                v.push((
                    x,
                    z,
                    self.right
                        .row(z as usize)
                        .iter()
                        .enumerate()
                        .filter_map(|(y, &right)| if right != !0 { Some(y as u8) } else { None })
                        .collect(),
                ));
            }
        }
        v
    }

    pub fn make_yet_points(&self, hole: &[Hole]) -> YetPointSet {
        let mut yet_yet = SmallVec::new();
        let mut yet = SmallVec::new();
        let mut can = SmallVec::new();
        for (x, z, yy) in hole.iter() {
            let x = *x;
            let z = *z;
            let front = self.front[(x, z)];
            for &y in yy.iter() {
                let right = self.right[(y, z)];
                let p = Point::new(x, y, z);
                match (front, right) {
                    (0, 0) => yet_yet.push(p),
                    (0, _) | (_, 0) => {
                        if yet_yet.is_empty() {
                            yet.push(p);
                        }
                    }
                    _ => {
                        if yet_yet.is_empty() && yet.is_empty() && self.grid[p] == 0 {
                            can.push(p);
                        }
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

fn grow_shared_block(
    rng: &mut Mcg128Xsl64,
    grid_1: &mut GridBox,
    grid_2: &mut GridBox,
    block_id: u16,
    p1: Point,
    p2: Point,
) -> (Vec<Point>, Vec<Point>) {
    let d = grid_1.d;
    let mut directions1 = [0, 1, 2, 3, 4, 5];
    let mut directions2 = [0, 1, 2, 3, 4, 5];
    directions1.shuffle(rng);
    directions2.shuffle(rng);
    let mut axis_map = AxisMap::new();
    let mut pp1 = Vec::with_capacity(4);
    let mut pp2 = Vec::with_capacity(4);
    let mut stack: SmallVec<[_; 32]> = smallvec![(p1, p2)];
    grid_1.put(p1, block_id);
    grid_2.put(p2, block_id);
    pp1.push(p1);
    pp2.push(p2);
    while let Some((p1, p2)) = stack.pop() {
        for &dir1 in directions1.iter() {
            if let Some(p1) = p1.next_cell(d, dir1) {
                if grid_1.grid[p1] != 0 {
                    continue;
                }
                for dir2 in axis_map.map_axis(dir1, directions2) {
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
    hole_1: &[Hole],
    hole_2: &[Hole],
    grid_1: &mut GridBox,
    grid_2: &mut GridBox,
    block: &mut BlockSet,
    cut_off: f64,
) -> Option<f64> {
    fn single_update_loop(
        grid: &mut GridBox,
        p: Point,
        block: &mut BlockSet,
        cut_off: f64,
        place: u8,
    ) -> f64 {
        if cut_off <= 2.0 {
            return 2.0;
        }
        let block_id = block.gen_half_block_id();
        let mut c = 1.0;
        let mut stack = vec![p];
        grid.put(p, block_id);
        block.push_half(place, p);
        'OUT: while let Some(p) = stack.pop() {
            for dir in 0..6 {
                if let Some(p) = p.next_cell(grid.d, dir) {
                    if grid.grid[p] == 0 {
                        grid.put(p, block_id);
                        block.push_half(place, p);
                        c += 1.0;
                        if c + 1.0 / c >= cut_off {
                            break 'OUT;
                        }
                        stack.push(p);
                    }
                }
            }
        }
        c + 1.0 / c
    }

    let mut score = 0.0;
    loop {
        if score >= cut_off {
            return None;
        }
        let yet1 = grid_1.make_yet_points(hole_1);
        let yet2 = grid_2.make_yet_points(hole_2);
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
                score += single_update_loop(grid_1, p, block, cut_off - score, 1);
            }
            (None, Some(p)) => {
                score += single_update_loop(grid_2, p, block, cut_off - score, 2);
            }
            (None, None) => return None,
        }
    }
    Some(score)
}

pub fn sa_run(
    start: Instant,
    limit: Duration,
    rng: &mut Mcg128Xsl64,
    hole_1: &[Hole],
    hole_2: &[Hole],
    grid_1: &mut GridBox,
    grid_2: &mut GridBox,
    block: &mut BlockSet,
    best: &mut SolveResult,
    params: McParams,
) -> u32 {
    let mut score = 1e100;
    let mut elapsed = start.elapsed();
    let mut step = 0;
    let mut need_erase = true;
    loop {
        step += 1;
        elapsed = if step % 256 == 0 {
            start.elapsed()
        } else {
            elapsed
        };
        if elapsed > limit {
            break step;
        }
        let temperature = params.temperature(limit, elapsed);

        if need_erase {
            for &p in block.half1.iter() {
                grid_1.remove(p);
            }
            for &p in block.half2.iter() {
                grid_2.remove(p);
            }
            block.half_reset();
            if params.erase_small_th > 0 {
                while let Some((p1, p2)) = block.pop_small(params.erase_small_th) {
                    for p in p1 {
                        grid_1.remove(p);
                    }
                    for p in p2 {
                        grid_2.remove(p);
                    }
                }
            }
        }
        let before_state = (grid_1.clone(), grid_2.clone(), block.clone());

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
        let cut_off = temperature * params.cut_off - sos + score;
        let new_score =
            sos + fill_all(rng, hole_1, hole_2, grid_1, grid_2, block, cut_off).unwrap_or(1e100);
        if new_score < score || rng.gen_bool(((score - new_score) / temperature).exp()) {
            score = new_score;
            best.set_best(grid_1, grid_2, score);
            need_erase = true;
        } else {
            *grid_1 = before_state.0;
            *grid_2 = before_state.1;
            *block = before_state.2;
            need_erase = false;
        }
    }
}

pub struct SolveInput {
    pub start: Instant,
    pub limit: Duration,
    pub front1: Vec<Vec<u8>>,
    pub right1: Vec<Vec<u8>>,
    pub front2: Vec<Vec<u8>>,
    pub right2: Vec<Vec<u8>>,
    pub params: McParams,
}

pub struct SolveResult {
    pub g1: Vec<u16>,
    pub g2: Vec<u16>,
    pub score: f64,
    pub run_count: u32,
}

impl SolveResult {
    pub fn worst() -> SolveResult {
        SolveResult {
            g1: Vec::new(),
            g2: Vec::new(),
            score: 1e300,
            run_count: 0,
        }
    }

    pub fn set_best(&mut self, g1: &GridBox, g2: &GridBox, score: f64) -> bool {
        if score < self.score {
            self.g1 = g1.grid.data.clone();
            self.g2 = g2.grid.data.clone();
            self.score = score;
            true
        } else {
            false
        }
    }
}

pub fn mc_solve(rng: &mut Mcg128Xsl64, input: &SolveInput, d: u8) -> SolveResult {
    let mut grid_1 = GridBox::new(d, &input.front1, &input.right1);
    let mut grid_2 = GridBox::new(d, &input.front2, &input.right2);
    let hole_1 = grid_1.make_hole_xzy();
    let hole_2 = grid_2.make_hole_xzy();
    let mut block = BlockSet::new();

    let mut best = SolveResult::worst();
    for i in 0..input.params.mc_run {
        let rest_run = input.params.mc_run - i;
        let total_mill = (input.limit - input.start.elapsed()).as_millis() as u64;
        let sub_limit = Duration::from_millis(total_mill / rest_run);
        let step = sa_run(
            Instant::now(),
            sub_limit,
            rng,
            &hole_1,
            &hole_2,
            &mut grid_1,
            &mut grid_2,
            &mut block,
            &mut best,
            input.params,
        );
        grid_1.reset(&hole_1);
        grid_2.reset(&hole_2);
        block.reset();
        best.run_count += step;
    }
    best
}
