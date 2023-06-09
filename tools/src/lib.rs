use proconio::{input, marker::Bytes};
use rand::prelude::*;

pub trait SetMinMax {
    fn setmin(&mut self, v: Self) -> bool;
    fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMinMax for T
where
    T: PartialOrd,
{
    fn setmin(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn setmax(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}

#[macro_export]
macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

#[derive(Clone, Debug)]
pub struct Output {
    pub n: usize,
    pub b: Vec<Vec<Vec<Vec<usize>>>>,
}

#[derive(Clone, Debug)]
pub struct Input {
    pub d: usize,
    pub f: Vec<Vec<Vec<i32>>>,
    pub r: Vec<Vec<Vec<i32>>>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.d)?;
        for i in 0..2 {
            for z in 0..self.d {
                for x in 0..self.d {
                    write!(f, "{}", self.f[i][z][x])?;
                }
                writeln!(f)?;
            }
            for z in 0..self.d {
                for x in 0..self.d {
                    write!(f, "{}", self.r[i][z][x])?;
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

pub fn parse_input(f: &str) -> Input {
    let mut f = proconio::source::once::OnceSource::from(f);
    input! {
        from &mut f,
        d: usize,
    }
    let mut fs = mat![0; 2; d; d];
    let mut rs = mat![0; 2; d; d];
    for i in 0..2 {
        input! {
            from &mut f,
            f: [Bytes; d],
            r: [Bytes; d],
        }
        for z in 0..d {
            for x in 0..d {
                fs[i][z][x] = (f[z][x] - b'0') as i32;
                rs[i][z][x] = (r[z][x] - b'0') as i32;
            }
        }
    }
    Input { d, f: fs, r: rs }
}

fn read<T: Copy + PartialOrd + std::fmt::Display + std::str::FromStr>(
    token: Option<&str>,
    lb: T,
    ub: T,
) -> Result<T, String> {
    if let Some(v) = token {
        if let Ok(v) = v.parse::<T>() {
            if v < lb || ub < v {
                Err(format!("Out of range: {}", v))
            } else {
                Ok(v)
            }
        } else {
            Err(format!("Parse error: {}", v))
        }
    } else {
        Err("Unexpected EOF".to_owned())
    }
}

pub fn parse_output(input: &Input, f: &str) -> Result<Output, String> {
    let mut b = mat![0; 2; input.d; input.d; input.d];
    let mut tokens = f.split_whitespace();
    let n = read(tokens.next(), 0, 1000000)?;
    for i in 0..2 {
        for x in 0..input.d {
            for y in 0..input.d {
                for z in 0..input.d {
                    b[i][x][y][z] = read(tokens.next(), 0, n)?;
                }
            }
        }
    }
    if tokens.next().is_some() {
        return Err("Too many outputs".to_owned());
    }
    Ok(Output { n, b })
}

fn normalize(b: &[(usize, usize, usize)]) -> Vec<(usize, usize, usize)> {
    let mut min_x = !0;
    let mut min_y = !0;
    let mut min_z = !0;
    for &(x, y, z) in b {
        min_x.setmin(x);
        min_y.setmin(y);
        min_z.setmin(z);
    }
    b.iter()
        .map(|&(x, y, z)| (x - min_x, y - min_y, z - min_z))
        .collect()
}

fn is_same(b1: &[(usize, usize, usize)], b2: &[(usize, usize, usize)]) -> bool {
    if b1.len() != b2.len() {
        return false;
    }
    let b1 = {
        let mut b1 = normalize(b1);
        b1.sort();
        b1
    };
    let mut b2 = normalize(b2);
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;
    for &(x, y, z) in &b2 {
        max_x.setmax(x);
        max_y.setmax(y);
        max_z.setmax(z);
    }
    for i in 0..6 {
        for _ in 0..4 {
            b2.sort();
            if b1 == b2 {
                return true;
            }
            for (x, y, _) in &mut b2 {
                let t = *x;
                *x = max_y - *y;
                *y = t;
            }
            std::mem::swap(&mut max_x, &mut max_y);
        }
        if i & 1 != 0 {
            for (_, y, z) in &mut b2 {
                let t = *y;
                *y = max_z - *z;
                *z = t;
            }
            std::mem::swap(&mut max_y, &mut max_z);
        } else {
            for (x, _, z) in &mut b2 {
                let t = *z;
                *z = max_x - *x;
                *x = t;
            }
            std::mem::swap(&mut max_x, &mut max_z);
        }
    }
    false
}

pub const D2: [(usize, usize); 4] = [(0, !0), (0, 1), (!0, 0), (1, 0)];
pub const D3: [(usize, usize, usize); 6] = [
    (0, 0, !0),
    (0, 0, 1),
    (0, !0, 0),
    (0, 1, 0),
    (!0, 0, 0),
    (1, 0, 0),
];

pub fn compute_score(input: &Input, out: &Output) -> (i64, String) {
    let mut pos = mat![vec![]; 2; out.n];
    let mut visited = mat![false; 2; input.d; input.d; input.d];
    for i in 0..2 {
        let mut f = mat![0; input.d; input.d];
        let mut r = mat![0; input.d; input.d];
        for x in 0..input.d {
            for y in 0..input.d {
                for z in 0..input.d {
                    let id = out.b[i][x][y][z];
                    if id != 0 {
                        f[z][x] = 1;
                        r[z][y] = 1;
                        pos[i][id - 1].push((x, y, z));
                        if pos[i][id - 1].len() == 1 {
                            visited[i][x][y][z] = true;
                            let mut stack = vec![(x, y, z)];
                            while let Some((x, y, z)) = stack.pop() {
                                for &(dx, dy, dz) in &D3 {
                                    let x2 = x + dx;
                                    let y2 = y + dy;
                                    let z2 = z + dz;
                                    if x2 < input.d
                                        && y2 < input.d
                                        && z2 < input.d
                                        && out.b[i][x2][y2][z2] == id
                                        && !visited[i][x2][y2][z2]
                                    {
                                        visited[i][x2][y2][z2] = true;
                                        stack.push((x2, y2, z2));
                                    }
                                }
                            }
                        } else if !visited[i][x][y][z] {
                            return (
                                0,
                                format!("block {} is not connected in the object {}", id, i + 1),
                            );
                        }
                    }
                }
            }
        }
        if f != input.f[i] {
            return (
                0,
                format!("The front silhouette for object {} does not match.", i + 1),
            );
        }
        if r != input.r[i] {
            return (
                0,
                format!("The right silhouette for object {} does not match.", i + 1),
            );
        }
    }
    let mut sum = 0.0f64;
    for i in 0..out.n {
        if pos[0][i].len() == 0 && pos[1][i].len() == 0 {
            return (0, format!("block {} is not used", i + 1));
        } else if pos[0][i].len() == 0 || pos[1][i].len() == 0 {
            sum += (pos[0][i].len() + pos[1][i].len()) as f64;
        } else if is_same(&pos[0][i], &pos[1][i]) {
            sum += 1.0 / pos[0][i].len() as f64;
        } else {
            return (
                0,
                format!(
                    "The shape of block {} differs between objects 1 and 2.",
                    i + 1
                ),
            );
        }
    }
    let score = (1e9 * sum).round() as i64;
    (score, String::new())
}

pub fn gen(seed: u64, custom_d: Option<usize>) -> Input {
    if seed == 0 {
        return parse_input(
            r#"5
10001
11011
11111
10101
10001
01110
11011
10000
11011
01110
11110
00011
01110
11000
11111
11110
00011
01110
00011
11110
"#,
        );
    }
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    let mut d = rng.gen_range(5i32, 15) as usize;
    if let Some(custom_d) = custom_d {
        d = custom_d;
    }
    let mut f = mat![0; 2; d; d];
    let mut r = mat![0; 2; d; d];
    let mut p = vec![0.0; 5];
    for dir in 1..5 {
        p[dir] = (d as f64).powf(rng.gen_range(-1.0, 1.0) + if dir >= 3 { 0.5 } else { 0.0 });
    }
    for i in 0..4 {
        let g = if i % 2 == 0 {
            &mut f[i / 2]
        } else {
            &mut r[i / 2]
        };
        loop {
            let num = rng.gen_range(d as i32 * 2, (d * d / 2) as i32 + 1);
            for z in 0..d {
                for x in 0..d {
                    g[z][x] = 0;
                }
            }
            let mut deg = mat![0; d; d];
            for _ in 0..num {
                let mut ws = vec![];
                for z in 0..d {
                    for x in 0..d {
                        if g[z][x] == 0 && deg[z][x] > 0 {
                            ws.push((z, x, p[deg[z][x]]));
                        }
                    }
                }
                let (z, x, _) = if ws.len() > 0 {
                    *ws.choose_weighted(&mut rng, |&(_, _, w)| w).unwrap()
                } else {
                    (
                        rng.gen_range(0, d as i32) as usize,
                        rng.gen_range(0, d as i32) as usize,
                        0.0,
                    )
                };
                g[z][x] = 1;
                for &(dz, dx) in &D2 {
                    let z2 = z + dz;
                    let x2 = x + dx;
                    if z2 < d && x2 < d {
                        deg[z2][x2] += 1;
                    }
                }
            }
            let mut ok = true;
            for z in 0..d {
                ok &= g[z].iter().any(|&v| v == 1);
            }
            if ok {
                break;
            }
        }
    }
    Input { d, f, r }
}
