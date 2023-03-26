use crate::Point;

#[derive(Clone)]
pub struct BlockSet {
    pub shared: Vec<(Vec<Point>, Vec<Point>)>,
    next_half_id: u16,
}

impl Default for BlockSet {
    fn default() -> Self {
        BlockSet {
            shared: Vec::new(),
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
        self.next_half_id = 10000;
    }

    pub fn shared_only_score(&self) -> f64 {
        let mut score = 0.0;
        for (v, _) in self.shared.iter() {
            score += 1.0 / v.len() as f64;
        }
        score
    }

    pub fn gen_shared_block_id(&mut self) -> u16 {
        (self.shared.len() + 1) as u16
    }

    pub fn gen_half_block_id(&mut self) -> u16 {
        let id = self.next_half_id;
        self.next_half_id += 1;
        id
    }

    pub fn push_shared(&mut self, pp1: Vec<Point>, pp2: Vec<Point>) {
        self.shared.push((pp1, pp2));
    }
}
