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
        let vp_s: Viewport = (x, 0, w, (h as i64 + y) as usize);
        let vp_n: Viewport = (x, y, w, y.abs() as usize);
        let south_xs = hemisphere(ss, Dir::Normal, vp_s);
        let mut north_xs = hemisphere(ns, Dir::Reverse, vp_n);
        north_xs.extend(south_xs);
        north_xs
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Dir {
    Normal,
    Reverse,
}
impl Dir {
    fn organize<T>(self, xs: Vec<T>) -> Vec<T> {
        if self == Dir::Reverse {
            xs.into_iter().rev().collect()
        } else {
            xs
        }
    }
}

fn hemisphere(master_seed: u64, dir: Dir, (x, y, w, h): Viewport) -> Vec<Vec<u64>> {
    let skips = y.abs() as usize;

    let (e_seeds, w_seeds): (Vec<_>, Vec<_>) = PsudoRng::new(master_seed)
        .skip(skips * 2)
        .take(h * 2)
        .enumerate()
        .partition(|(i, _v)| i % 2 == 0);

    let seed_pairs = e_seeds.iter().zip(w_seeds.iter());

    let xs: Vec<Vec<u64>> = seed_pairs
        .map(|((_i, e_seed), (_j, w_seed))| Longitudes::take_finite((*e_seed, *w_seed), x, w))
        .collect();

    dir.organize(xs)
}

// Infinite "line of numbers"
// going from center (west[0]) out into 2 directions"
pub struct Longitudes {}

impl Longitudes {
    pub fn take_finite((e_seed, w_seed): Seeds, x: i64, w: usize) -> Vec<u64> {
        let skips = x.abs() as usize;
        //println!("Skips = {}", skips);
        if x >= 0 {
            // westward
            Longitudes::take_from_one_dir(Dir::Normal, w_seed, skips, w)
        } else if (x.abs() as usize) >= w {
            // eastward .rev[erse]
            Longitudes::take_from_one_dir(Dir::Reverse, e_seed, skips, w)
        } else {
            // bordering
            let west_xs =
                Longitudes::take_from_one_dir(Dir::Normal, w_seed, 0, (w as i64 + x) as usize);
            let mut east_xs = Longitudes::take_from_one_dir(Dir::Reverse, e_seed, skips, skips);
            east_xs.extend(west_xs);
            east_xs
        }
    }

    fn take_from_one_dir(dir: Dir, seed: u64, skips: usize, w: usize) -> Vec<u64> {
        let rng = PsudoRng::new(seed);
        let xs: Vec<u64> = rng.skip(skips).take(w).collect();
        dir.organize(xs)
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
