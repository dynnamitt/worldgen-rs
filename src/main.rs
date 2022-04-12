mod another;
use another::*;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::time::*;

// wrap a Trait
struct RandIntGen(Box<dyn RngCore>);

impl RandIntGen {
    // class method (::)
    fn new(seed: u64) -> RandIntGen {
        RandIntGen(Box::new(ChaCha8Rng::seed_from_u64(seed)))
    }
}

impl Iterator for RandIntGen {
    type Item = u32;
    // obj method (.)
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0.next_u32())
    }
}

fn fraction(i: u32) -> f64 {
    let max = f64::from(u32::MAX);
    let i_ = f64::from(i);
    i_ % max / max
}

fn main() {
    let now = Instant::now();

    RandIntGen::new(2121)
        .take(222 * 80)
        .map(fraction)
        .for_each(|i| print!("{}, ", i));

    let elapsed1 = now.elapsed();

    println!("\n took ==> {} ", elapsed1.as_millis());
}
