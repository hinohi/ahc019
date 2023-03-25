use crate::{AxisMap, BlockSet, Grid3, GridFront, GridRight, McParams, Point};
use rand::{seq::SliceRandom, Rng};
use rand_pcg::Mcg128Xsl64;
use std::time::{Duration, Instant};

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
        let front = GridFront::from_vec(d, make_face(&front, false));
        let right = GridRight::from_vec(d, make_face(&right, true));
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

    pub fn make_hole_xzy(&self) -> Vec<(u8, u8, Vec<u8>)> {
        let mut v = Vec::new();
        for x in 0..self.d {
            for z in 0..self.d {
                let front = self.front.data[(x * self.d + z) as usize];
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

    pub fn make_yet_points(&self, hole: &[(u8, u8, Vec<u8>)]) -> YetPointSet {
        let mut yet_yet = Vec::new();
        let mut yet = Vec::new();
        let mut can = Vec::new();
        for (x, z, yy) in hole.iter() {
            let x = *x;
            let z = *z;
            let front = self.front.data[(x * self.d + z) as usize];
            for &y in yy.iter() {
                let right = self.right.data[(z * self.d + y) as usize];
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
    let mut stack = vec![(p1, p2)];
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
                for dir2 in axis_map.map_axis(dir1, &directions2) {
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
    hole_1: &[(u8, u8, Vec<u8>)],
    hole_2: &[(u8, u8, Vec<u8>)],
    grid_1: &mut GridBox,
    grid_2: &mut GridBox,
    block: &mut BlockSet,
    cut_off: f64,
) -> Option<f64> {
    fn single_update_loop(grid: &mut GridBox, p: Point, block: &mut BlockSet, place: u8) -> f64 {
        let block_id = block.gen_half_block_id();
        let mut c = 0;
        let mut stack = vec![p];
        grid.put(p, block_id);
        block.push_half(place, p);
        c += 1;
        while let Some(p) = stack.pop() {
            for dir in 0..6 {
                if let Some(p) = p.next_cell(grid.d, dir) {
                    if grid.grid[p] == 0 {
                        grid.put(p, block_id);
                        block.push_half(place, p);
                        c += 1;
                        stack.push(p);
                    }
                }
            }
        }
        1.0 / c as f64 + c as f64
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
                score += single_update_loop(grid_1, p, block, 1);
            }
            (None, Some(p)) => {
                score += single_update_loop(grid_2, p, block, 2);
            }
            (None, None) => return None,
        }
    }
    Some(score)
}

pub fn mc_run(
    start: Instant,
    limit: Duration,
    rng: &mut Mcg128Xsl64,
    d: u8,
    front1: &[Vec<u8>],
    right1: &[Vec<u8>],
    front2: &[Vec<u8>],
    right2: &[Vec<u8>],
    params: McParams,
) -> ((Vec<u16>, Vec<u16>, f64), u32) {
    let mut grid_1 = GridBox::new(d, &front1, right1);
    let mut grid_2 = GridBox::new(d, &front2, right2);
    let hole_1 = grid_1.make_hole_xzy();
    let hole_2 = grid_2.make_hole_xzy();
    let mut block = BlockSet::new();
    let mut best = loop {
        if let Some(score) = fill_all(
            rng,
            &hole_1,
            &hole_2,
            &mut grid_1,
            &mut grid_2,
            &mut block,
            1e100,
        ) {
            break (grid_1.grid.data.clone(), grid_2.grid.data.clone(), score);
        }
        grid_1 = GridBox::new(d, &front1, right1);
        grid_2 = GridBox::new(d, &front2, right2);
        block = BlockSet::new();
    };
    let mut score = best.2;
    let mut elapsed = start.elapsed();
    let mut step = 0;
    loop {
        step += 1;
        elapsed = if step % 256 == 0 {
            start.elapsed()
        } else {
            elapsed
        };
        if elapsed > limit {
            break (best, step);
        }
        let temperature = params.temperature(limit, elapsed);

        let before_state = (grid_1.clone(), grid_2.clone(), block.clone());

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
        let new_score = sos
            + fill_all(
                rng,
                &hole_1,
                &hole_2,
                &mut grid_1,
                &mut grid_2,
                &mut block,
                cut_off,
            )
            .unwrap_or(1e100);
        if new_score < score || rng.gen_bool(((score - new_score) / temperature).exp()) {
            score = new_score;
            if score < best.2 {
                best = (grid_1.grid.data.clone(), grid_2.grid.data.clone(), score);
            }
        } else {
            grid_1 = before_state.0;
            grid_2 = before_state.1;
            block = before_state.2;
        }
    }
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

    pub fn set_best(&mut self, g1: Vec<u16>, g2: Vec<u16>, score: f64, mc: u32) -> bool {
        self.run_count += mc;
        if score < self.score {
            self.g1 = g1;
            self.g2 = g2;
            self.score = score;
            true
        } else {
            false
        }
    }
}

pub fn mc_solve(
    start: Instant,
    limit: Duration,
    rng: &mut Mcg128Xsl64,
    d: u8,
    front1: &[Vec<u8>],
    right1: &[Vec<u8>],
    front2: &[Vec<u8>],
    right2: &[Vec<u8>],
    params: McParams,
) -> SolveResult {
    let total_mill = (limit - start.elapsed()).as_millis() as u64;
    let sub_limit = Duration::from_millis(total_mill / params.mc_run);
    let mut best = SolveResult::worst();
    for _ in 0..params.mc_run {
        let ((g1, g2, score), step) = mc_run(
            Instant::now(),
            sub_limit,
            rng,
            d,
            front1,
            right1,
            front2,
            right2,
            params,
        );
        best.set_best(g1, g2, score, step);
    }
    best
}
