use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

pub type Viewport = (i64, i64, u16, u16);
pub type Seeds = (u64, u64);

type IntGrid = Vec<Vec<u64>>;

#[allow(dead_code)]
pub fn world((ns, ss): Seeds, vp @ (x, y, w, h): Viewport) -> IntGrid {
    if y >= 0 {
        // southward
        hemisphere(ss, Direction::Normal, vp)
    } else if y.abs() >= h as i64 {
        // northward .rev[erse]
        hemisphere(ns, Direction::Reverse, vp)
    } else {
        // bordering
        let vp_s: Viewport = (x, 0, w, (h as i64 + y) as u16);
        let vp_n: Viewport = (x, y, w, y.abs() as u16);
        let south_xs = hemisphere(ss, Direction::Normal, vp_s);
        let north_xs = hemisphere(ns, Direction::Reverse, vp_n);
        north_xs.into_iter().chain(south_xs.into_iter()).collect()
    }
}

#[allow(dead_code)]
pub fn redist_with_zoom(grid: IntGrid, (_, _, w, h): Viewport, z: u16) -> IntGrid {
    if z == 1 {
        grid
    } else {
        (0..(h * z) - 1)
            .into_iter()
            .map(|y| {
                (0..(w * z) - 1)
                    .into_iter()
                    .map(|x| grid[(y / z) as usize][(x / z) as usize])
                    .collect()
            })
            .collect()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    Normal,
    Reverse,
}
impl Direction {
    fn organize<T>(self, xs: Vec<T>) -> Vec<T> {
        if self == Direction::Reverse {
            xs.into_iter().rev().collect()
        } else {
            xs
        }
    }
}

fn hemisphere(master_seed: u64, dir: Direction, (x, y, w, h): Viewport) -> Vec<Vec<u64>> {
    let skips = y.abs() as usize;

    let (e_seeds, w_seeds): (Vec<_>, Vec<_>) = PsudoRng::new(master_seed)
        .skip(skips * 2)
        .take(h as usize * 2)
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
    pub fn take_finite((e_seed, w_seed): Seeds, x: i64, w: u16) -> Vec<u64> {
        let skips = x.abs() as usize;
        //println!("Skips = {}", skips);
        if x >= 0 {
            // westward
            Longitudes::take_from_one_dir(Direction::Normal, w_seed, skips, w)
        } else if x.abs() >= w as i64 {
            // eastward .rev[erse]
            Longitudes::take_from_one_dir(Direction::Reverse, e_seed, skips, w)
        } else {
            // bordering
            let west_xs =
                Longitudes::take_from_one_dir(Direction::Normal, w_seed, 0, (w as i64 + x) as u16);
            // TODO use iter.chain here also
            let mut east_xs =
                Longitudes::take_from_one_dir(Direction::Reverse, e_seed, skips, skips as u16);
            east_xs.extend(west_xs);
            east_xs
        }
    }

    fn take_from_one_dir(dir: Direction, seed: u64, skips: usize, w: u16) -> Vec<u64> {
        let rng = PsudoRng::new(seed);
        let xs: Vec<u64> = rng.skip(skips).take(w as usize).collect();
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
