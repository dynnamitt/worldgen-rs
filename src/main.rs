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
struct Longitude {
    east: PsudoRng,
    west: PsudoRng,
}

impl Longitude {
    fn new(e_seed: u64, w_seed: u64) -> Self {
        let eg = PsudoRng::new(e_seed);
        let wg = PsudoRng::new(w_seed);
        Self { east: eg, west: wg }
    }

    fn take_finite(self, x: i64, len: usize) -> Vec<u64> {
        if x >= 0 {
            self.west.into_iter().skip(x as usize).take(len).collect()
        } else if (x.abs() as usize) >= len {
            self.east
                .skip(x.abs() as usize - 1)
                .take(len)
                .collect::<Vec<u64>>()
                .into_iter()
                .rev()
                .collect()
        } else {
            todo!()
        }
    }
}

struct WorldGrid(u64, u64);

//
//  Take out a finite vec of Longitudes
//  Select an Y-pos where
//     0  == first southward long.
//     -1 == first northward long.
impl WorldGrid {
    fn longitudes(self, y: i64, h: usize, x: i64, len: usize) -> Vec<Vec<u64>> {
        if y >= 0 {
            let (south_east_xs, south_west_xs): (Vec<_>, Vec<_>) = PsudoRng::new(self.1)
                .skip(y as usize * 2)
                .take(h * 2)
                .enumerate()
                .partition(|(i, _)| i % 2 == 0);

            south_east_xs
                .iter()
                .zip(south_west_xs)
                .map(|((_, east_s), (_, west_s))| {
                    Longitude::new(*east_s, west_s).take_finite(x, len)
                })
                //.flatten()
                .collect()
        } else {
            todo!()
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
    let h: usize = 7;
    let grid = w.longitudes(0, h, 0, 3);
    for g in grid.iter().enumerate() {
        println!(" {:?}", g);
    }

    println!(" ---- ");

    g.take(2 * 2).map(fraction).for_each(|i| print!("{} / ", i));

    let elapsed1 = now.elapsed();

    println!("\n took ==> {} ", elapsed1.as_millis());
}
