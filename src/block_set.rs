use crate::Point;
use rand::Rng;
use rand_pcg::Mcg128Xsl64;

#[derive(Clone)]
pub struct BlockSet {
    pub shared: Vec<(u16, Vec<Point>, Vec<Point>)>,
    shared_id_stock: Vec<u16>,
    pub half1: Vec<Point>,
    pub half2: Vec<Point>,
    next_half_id: u16,
}

impl Default for BlockSet {
    fn default() -> Self {
        BlockSet {
            shared: Vec::new(),
            shared_id_stock: Vec::new(),
            half1: Vec::new(),
            half2: Vec::new(),
            next_half_id: 10000,
        }
    }
}

impl BlockSet {
    pub fn new() -> BlockSet {
        Default::default()
    }

    pub fn reset(&mut self) {
        self.shared.clear();
        self.shared_id_stock.clear();
        self.half1.clear();
        self.half2.clear();
        self.next_half_id = 10000;
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
