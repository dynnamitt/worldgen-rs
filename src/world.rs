use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

pub type Viewport = (i64, i64, usize, usize);
pub type Seeds = (u64, u64);

pub fn world((ns, ss): Seeds, vp @ (x, y, w, h): Viewport) -> Vec<Vec<u64>> {
    if y >= 0 {
        // southward
        hemisphere(ss, Dir::Normal, vp)
    } else if (y.abs() as usize) >= h {
        // northward .rev[erse]
        hemisphere(ns, Dir::Reverse, vp)
    } else {
        // bordering
        println!(" y:{}  h:{}  .. ", y, h);
        let vp_s: Viewport = (x, 0, w, (h as i64 + y) as usize);
        let vp_n: Viewport = (x, y, w, y.abs() as usize);
        let south_xs = hemisphere(ss, Dir::Normal, vp_s);
        let mut north_xs = hemisphere(ns, Dir::Reverse, vp_n);
        north_xs.extend(south_xs);
        north_xs
    }
}

#[derive(PartialEq, Eq)]
pub enum Dir {
    Normal,
    Reverse,
}

fn hemisphere(master_seed: u64, dir: Dir, (x, y, w, h): Viewport) -> Vec<Vec<u64>> {
    let skips = y.abs() as usize;

    let (e_seeds, w_seeds): (Vec<_>, Vec<_>) = PsudoRng::new(master_seed)
        .skip(skips * 2)
        .take(h * 2)
        .enumerate()
        .partition(|(i, _v)| i % 2 == 0);

    let seed_pairs = e_seeds.iter().zip(w_seeds);

    let xs: Vec<Vec<u64>> = seed_pairs
        .map(|((_i, e_seed), (_j, w_seed))| Longitude::new(*e_seed, w_seed).take_finite(x, w))
        .collect();

    if dir == Dir::Normal {
        xs
    } else {
        xs.into_iter().rev().collect()
    }
}

// Infinite "line of numbers"
// going from center (west[0]) out into 2 directions"
pub struct Longitude {
    east: PsudoRng,
    west: PsudoRng,
}

impl Longitude {
    pub fn new(e_seed: u64, w_seed: u64) -> Self {
        let eg = PsudoRng::new(e_seed);
        let wg = PsudoRng::new(w_seed);
        Self { east: eg, west: wg }
    }
    pub fn take_finite(self, x: i64, len: usize) -> Vec<u64> {
        if x >= 0 {
            // westward
            self.west.into_iter().skip(x as usize).take(len).collect()
        } else if (x.abs() as usize) >= len {
            // eastward .rev[erse]
            self.east
                .skip(x.abs() as usize - 1)
                .take(len)
                .collect::<Vec<u64>>()
                .into_iter()
                .rev()
                .collect()
        } else {
            // bordering
            todo!()
        }
    }
}

// wrap a Trait
pub struct PsudoRng(Box<dyn RngCore>);

impl PsudoRng {
    // class method (::)
    pub fn new(seed: u64) -> PsudoRng {
        PsudoRng(Box::new(ChaCha8Rng::seed_from_u64(seed)))
    }
}

impl Iterator for PsudoRng {
    type Item = u64;
    // obj method (.)
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0.next_u64())
    }
}
