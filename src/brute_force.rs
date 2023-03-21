use crate::{AxisMap, Grid3, GridFront, GridRight, Point};
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Copy, Clone, Eq, PartialEq)]
enum FaceState {
    Yet,
    Satisfy,
    Null,
}

#[derive(Clone)]
struct GridBox {
    d: u8,
    grid: Grid3<u8>,
    front: GridFront<FaceState>,
    right: GridRight<FaceState>,
}

fn make_face(shadow: &[Vec<u8>]) -> Vec<FaceState> {
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
    pub fn new(d: u8, front: &[Vec<u8>], right: &[Vec<u8>]) -> GridBox {
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

    pub fn key(&self) -> u128 {
        let mut k = 0;
        for (i, &g) in self.grid.data.iter().enumerate() {
            if g == 0 {
                k ^= 1 << i;
            }
        }
        k
    }

    pub fn make_can_points(&self) -> Vec<Point> {
        let mut v = Vec::new();
        for x in 0..self.d {
            for y in 0..self.d {
                for z in 0..self.d {
                    let p = Point::new(x, y, z);
                    if self.grid[p] == 0 {
                        v.push(p);
                    }
                }
            }
        }
        v
    }

    pub fn is_satisfy(&self) -> bool {
        self.front.data.iter().all(|&f| f != FaceState::Yet)
            && self.right.data.iter().all(|&f| f != FaceState::Yet)
    }

    pub fn put(&mut self, p: Point, block_id: u8) {
        self.grid[p] = block_id;
        self.front[p] = FaceState::Satisfy;
        self.right[p] = FaceState::Satisfy;
    }
}

struct StackItem {
    p1: Point,
    p2: Point,
    axis: AxisMap,
}

fn grow_shared_block(
    grid_1: &GridBox,
    grid_2: &GridBox,
    block_id: u8,
    p1: Point,
    p2: Point,
) -> Vec<(GridBox, GridBox, u8)> {
    let d = grid_1.d;
    let mut grids = FxHashMap::default();
    grids.insert(AxisMap::new(), (grid_1.clone(), grid_2.clone(), 0));
    let mut stack = vec![StackItem {
        p1,
        p2,
        axis: AxisMap::new(),
    }];
    while let Some(item) = stack.pop() {
        {
            let (grid_1, grid_2, cell) = grids.get_mut(&item.axis).unwrap();
            if grid_1.grid[item.p1] == 0 && grid_2.grid[item.p2] == 0 {
                grid_1.put(item.p1, block_id);
                grid_2.put(item.p2, block_id);
                *cell += 1;
            }
        }
        let (grid_1, grid_2, c) = grids[&item.axis].clone();
        for dir1 in 0..6 {
            if let Some(p1) = item.p1.next_cell(d, dir1) {
                if grid_1.grid[p1] != 0 {
                    continue;
                }
                for dir2 in item.axis.map_axis(dir1, &[0, 1, 2, 3, 4, 5]) {
                    if let Some(p2) = item.p2.next_cell(d, dir2) {
                        if grid_2.grid[p2] == 0 {
                            let axis = item.axis.fix(dir1, dir2);
                            grids
                                .entry(axis)
                                .or_insert_with(|| (grid_1.clone(), grid_2.clone(), c));
                            stack.push(StackItem { p1, p2, axis });
                        }
                    }
                }
            }
        }
    }
    grids.into_values().collect()
}

pub struct SolveResult {
    pub g1: Vec<u16>,
    pub g2: Vec<u16>,
    pub score: f64,
}

pub fn solve(
    d: u8,
    front1: &[Vec<u8>],
    right1: &[Vec<u8>],
    front2: &[Vec<u8>],
    right2: &[Vec<u8>],
) -> SolveResult {
    let mut mem = FxHashSet::with_capacity_and_hasher(10000, Default::default());
    let mut grids = vec![(
        GridBox::new(d, front1, right1),
        GridBox::new(d, front2, right2),
        0.0,
        0,
    )];
    let mut best = grids[0].clone();
    best.2 = 1.0;
    while let Some((grid_1, grid_2, score, last_block_id)) = grids.pop() {
        if score >= best.2 {
            continue;
        }
        let block_id = last_block_id + 1;
        let pp1 = grid_1.make_can_points();
        let pp2 = grid_2.make_can_points();
        for &p1 in pp1.iter() {
            for &p2 in pp2.iter() {
                for (grid_1, grid_2, c) in grow_shared_block(&grid_1, &grid_2, block_id, p1, p2) {
                    let new_score = score + 1.0 / c as f64;
                    if new_score >= best.2 {
                        continue;
                    }
                    let key = (grid_1.key(), grid_2.key());
                    if !mem.insert(key) {
                        continue;
                    }
                    if grid_1.is_satisfy() && grid_2.is_satisfy() {
                        if new_score < best.2 {
                            best = (grid_1, grid_2, new_score, block_id);
                        }
                    } else {
                        grids.push((grid_1, grid_2, new_score, block_id));
                    }
                }
            }
        }
        eprintln!("{} {} {} {}", mem.len(), grids.len(), best.2, score);
    }
    SolveResult {
        g1: best
            .0
            .grid
            .data
            .iter()
            .map(|&g| if g != !0 { g as u16 } else { !0 })
            .collect(),
        g2: best
            .1
            .grid
            .data
            .iter()
            .map(|&g| if g != !0 { g as u16 } else { !0 })
            .collect(),
        score: best.2,
    }
}
