mod world;
use std::time::*;
use world::*;

fn fraction(i: u64) -> f64 {
    let max = u64::MAX as f64;
    i as f64 % max / max
}

fn main() {
    let now = Instant::now();

    let seeds = (33, 77777);
    for (x, y) in [(0, 0), (-3, -3), (-3, 0), (0, -3)] {
        let grid1 = world(seeds, (x, y, 3, 3));
        println!("Start pos {},{}", x, y);
        for row in grid1.iter().enumerate() {
            println!(" {:?}", row);
        }
    }

    println!("-- NOW BORDERING");
    for (x, y) in [(0, -3), (-3, 1)] {
        let grid1 = world(seeds, (x, y, 4, 4));
        println!("Start pos {},{}", x, y);
        for row in grid1.iter().enumerate() {
            println!(" {:?}", row);
        }
    }

    println!(" ----v-----------v------ ");

    let g = PsudoRng::new(2121);
    g.take(2 * 2).map(fraction).for_each(|i| print!("{} / ", i));

    let elapsed1 = now.elapsed();

    println!("\n took ==> {} ", elapsed1.as_millis());
}
