mod another;
//use another::*;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::time::*;

// wrap a Trait
struct PsudoRng(Box<dyn RngCore>);

impl PsudoRng {
    // class method (::)
    fn new(seed: u64) -> PsudoRng {
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

pub type Viewport = (i64, i64, usize, usize);

#[derive(PartialEq, Eq)]
pub enum Dir {
    Normal,
    Reverse,
}
pub struct WorldGrid(u64, u64);

impl WorldGrid {
    //  Take out a finite vec of Longitudes
    //  Select an Y-pos where
    //     0  == first southward long.
    //     -1 == first northward long.
    pub fn longitudes(self, vp @ (x, y, w, h): Viewport) -> Vec<Vec<u64>> {
        let n_seed = self.0;
        let s_seed = self.1;
        if y >= 0 {
            // southward
            WorldGrid::one_dir(s_seed, Dir::Normal, vp)
        } else if (y.abs() as usize) >= h {
            // northward .rev[erse]
            WorldGrid::one_dir(n_seed, Dir::Reverse, vp)
        } else {
            // bordering
            let vp_s = (x, y, w, h);
            let vp_n = (x, y, w, h);
            let south_xs = WorldGrid::one_dir(s_seed, Dir::Normal, vp_s);
            let mut north_xs = WorldGrid::one_dir(n_seed, Dir::Reverse, vp_n);
            north_xs.extend(south_xs);
            north_xs
        }
    }

    fn one_dir(master_seed: u64, dir: Dir, (x, y, w, h): Viewport) -> Vec<Vec<u64>> {
        let skips = y.abs() as usize;
        let skips = if dir == Dir::Normal { skips - 1 } else { skips };

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
}

fn fraction(i: u64) -> f64 {
    let max = u64::MAX as f64;
    i as f64 % max / max
}

fn main() {
    let now = Instant::now();

    let g = PsudoRng::new(2121);

    let w = WorldGrid(2021, 2022);

    let grid = w.longitudes((0, -10, 4, 4));
    for g in grid.iter().enumerate() {
        println!(" {:?}", g);
    }

    println!(" ---- ");

    g.take(2 * 2).map(fraction).for_each(|i| print!("{} / ", i));

    let elapsed1 = now.elapsed();

    println!("\n took ==> {} ", elapsed1.as_millis());
}
