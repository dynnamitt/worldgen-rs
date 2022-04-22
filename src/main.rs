mod utils;
mod world;
use std::thread::sleep;
use std::time::{Duration, Instant};
use termion::{clear, color, cursor, style, terminal_size};
use utils::*;
use world::*;

#[allow(dead_code)]

fn colorize_row(it: impl IntoIterator<Item = u64>) -> String {
    it.into_iter()
        .map(fraction)
        .map(|fr| color_utils::sample_by_frac(fr, color_utils::GRAYSCALE))
        .map(color::AnsiValue::grayscale)
        .map(|a| a.bg_string() + ".")
        .collect::<Vec<String>>()
        .join("")
}

const SEEDS: Seeds = (329_329_892_390, 32_309_302);

type WorldGen = Box<dyn Fn(i64, i64) -> Vec<Vec<u64>>>;

fn create_world_gen(cols: u16, rows: u16, z: u16) -> WorldGen {
    Box::new(move |x: i64, y: i64| {
        let w = div_ceil(cols, z);
        let h = div_ceil(rows, z);
        world(SEEDS, (x, y, w, h))
    })
}

fn main() {
    let (cols, rows) = terminal_size().unwrap(); // never use it :-p
    let pause = Duration::from_millis(1000);
    let nums = -5_i64..=5_i64;
    let zoom = 1;
    let wrld_gen: WorldGen = create_world_gen(cols, rows, zoom);
    println!("{}{}", clear::All, style::Reset);

    for i in nums {
        // let height = div_ceil(rows, z) + 2; // one xtra top+bottom
        // let width = div_ceil(cols, z) + 2; // one xtra left+right
        // let vp = (i, -i, width, height);

        //only change in steps EQ z
        let grid1 = wrld_gen(i, -i);
        let grid2 = redist_with_zoom(grid1, (cols, rows), zoom);

        print!("{}", cursor::Goto(1, 1));
        grid2
            .into_iter()
            .map(colorize_row)
            .for_each(|r| print!("{}", r));
        sleep(pause);
    }
    println!("{}", style::Reset);
}

#[allow(dead_code)]
mod color_utils {
    pub type ColorBounds = (u8, u8);
    pub const RGB: ColorBounds = (16, 232);
    pub const GRAYSCALE: ColorBounds = (232, 255);
    pub const BRIGHT: ColorBounds = (8, 16);
    pub const STD_LOW: ColorBounds = (0, 8);

    pub fn sample_by_frac(fr: f64, bs: ColorBounds) -> u8 {
        assert!(fr >= 0_f64 && fr <= 1_f64);
        let len = bs.1 - bs.0;
        let pos: u8 = (len as f64 * fr).round() as u8;
        pos
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
