mod world;
use std::time::*;
use termion::{clear, color, cursor, style, terminal_size};
use world::*;

#[allow(dead_code)]
fn fraction(i: u64) -> f64 {
    let max = u64::MAX as f64;
    i as f64 % max / max
}

fn colorize_row(it: impl IntoIterator<Item = u64>) -> String {
    it.into_iter()
        .map(fraction)
        .map(|fr| color_utils::AnsiColors::sample(fr, color_utils::GRAYSCALE))
        .map(color::AnsiValue::grayscale)
        .map(|a| a.bg_string() + " ")
        .collect::<Vec<String>>()
        .join("")
}

fn main() {
    println!("{}{}{}", clear::All, style::Reset, cursor::Goto(1, 1));
    let seeds = (329329892390, 32309302);
    let (cols, rows) = terminal_size().unwrap(); // never use it :-p

    for i in [-2, -1, 0, 1, 2, 3, 4, 5] {
        let vp = (-i, -i, cols as usize, rows as usize);
        let grid1 = world(seeds, vp);

        grid1
            .into_iter()
            .map(colorize_row)
            .for_each(|r| print!("{}", r));

        println!("{}", cursor::Goto(1, 1));
    }
}

#[allow(dead_code)]
mod color_utils {
    pub type ColorBounds = (u8, u8);
    pub const RGB: ColorBounds = (16, 232);
    pub const GRAYSCALE: ColorBounds = (232, 255);
    pub const BRIGHT: ColorBounds = (8, 16);
    pub const STD_LOW: ColorBounds = (0, 8);

    pub struct AnsiColors {}
    impl AnsiColors {
        pub fn sample(frac: f64, bs: ColorBounds) -> u8 {
            let len = bs.1 - bs.0;
            (len as f64 * frac) as u8
        }
    }
}

#[allow(dead_code)]
fn tests() {
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
    for (x, y) in [(0, -3), (-3, 0), (-3, -3)] {
        println!("Start pos {},{}", x, y);
        let grid1 = world(seeds, (x, y, 45, 45));
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
