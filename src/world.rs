use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

pub type Viewport = (i64, i64, u16, u16);
pub type Seeds = (u64, u64);

#[derive(Eq, PartialEq, Debug)]
enum RowOffset {
    Complete,
    CappedEnds,
}
impl RowOffset {
    fn from_usize(n: usize) -> Self {
        if n % 2 == 0 {
            Self::Complete
        } else {
            Self::CappedEnds
        }
    }
}

pub type Row<T> = (RowOffset, Vec<T>);
pub type IntGrid<T> = Vec<Row<T>>;

#[allow(dead_code)]
pub fn noise_2d((ns, ss): Seeds, vp @ (x, y, w, h): Viewport) -> IntGrid<u64> {
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

fn hemisphere(master_seed: u64, dir: Direction, (x, y, w, h): Viewport) -> IntGrid<u64> {
    let skips = y.abs() as usize;

    let (e_seeds, w_seeds): (Vec<_>, Vec<_>) = PsudoRng::new(master_seed)
        .skip(skips * 2)
        .take(h as usize * 2)
        .enumerate()
        .partition(|(i, _v)| i % 2 == 0);

    let seed_pairs = e_seeds.iter().zip(w_seeds.iter());

    let xs: Vec<_> = seed_pairs
        .map(|((_i, e_seed), (_j, w_seed))| Longitudes::take_finite((*e_seed, *w_seed), x, w))
        .collect();

    dir.organize(xs)
}

// Infinite "line of numbers"
// going from center (west[0]) out into 2 directions"
pub struct Longitudes {}

impl Longitudes {
    pub fn take_finite((e_seed, w_seed): Seeds, x: i64, w: u16) -> Row<u64> {
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

            assert_eq!(west_xs.0, east_xs.0); // same RowOffset ?

            (
                east_xs.0,
                east_xs.1.into_iter().chain(west_xs.1.into_iter()).collect(),
            )
        }
    }

    fn take_from_one_dir(dir: Direction, seed: u64, skips: usize, w: u16) -> Row<u64> {
        let rng = PsudoRng::new(seed);
        let xs: Vec<_> = rng.skip(skips).take(w as usize).collect();
        (RowOffset::from_usize(skips), dir.organize(xs))
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
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn world() {
        todo!();
    }
}

#[allow(dead_code)]
fn tests() {
    let seeds = (33, 77777);
    for (x, y) in [(0, 0), (-3, -3), (-3, 0), (0, -3)] {
        let grid1 = noise_2d(seeds, (x, y, 3, 3));
        println!("Start pos {},{}", x, y);
        for row in grid1.iter().enumerate() {
            println!(" {:?}", row);
        }
    }

    println!("-- NOW BORDERING");
    for (x, y) in [(0, -3), (-3, 0), (-3, -3)] {
        println!("Start pos {},{}", x, y);
        let grid1 = noise_2d(seeds, (x, y, 45, 45));
        for row in grid1.iter().enumerate() {
            println!(" {:?}", row);
        }
    }
}
