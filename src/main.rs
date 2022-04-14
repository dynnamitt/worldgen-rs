mod world;
use std::time::*;
use world::*;

fn fraction(i: u64) -> f64 {
    let max = u64::MAX as f64;
    i as f64 % max / max
}

fn main() {
    let now = Instant::now();

    let x = 0;
    let y = -4;
    let grid = world((101041010, 56565656), (x, y, 3, 5));
    for row in grid.iter().enumerate() {
        println!(" {:?}", row);
    }

    println!(" ----v-----------v------ ");

    let g = PsudoRng::new(2121);
    g.take(2 * 2).map(fraction).for_each(|i| print!("{} / ", i));

    let elapsed1 = now.elapsed();

    println!("\n took ==> {} ", elapsed1.as_millis());
}
