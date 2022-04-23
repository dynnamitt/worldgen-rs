mod utils;
mod world;
use std::thread::sleep;
use std::time::Duration;
use termion::{clear, color, cursor, style, terminal_size};
use utils::*;
use utils::{colors, math};
use world::*;

#[allow(dead_code)]

fn colorize_row(it: impl IntoIterator<Item = u64>) -> String {
    it.into_iter()
        .map(math::fraction)
        .map(|fr| colors::sample_by_frac(fr, colors::GRAYSCALE))
        .map(color::AnsiValue::grayscale)
        .map(|a| a.bg_string() + ".")
        .collect::<Vec<String>>()
        .join("")
}

const SEEDS: Seeds = (329_329_892_390, 32_309_302);

type WorldGen = Box<dyn Fn(i64, i64) -> Vec<Vec<u64>>>;

fn create_world_gen(cols: u16, rows: u16, z: u16) -> WorldGen {
    Box::new(move |x: i64, y: i64| {
        let w = math::div_ceil(cols, z) + 2; // buffer 2 for offset picking
        let h = math::div_ceil(rows, z) + 2; // "
        noise_2d(SEEDS, (x, y, w, h))
    })
}

fn main() {
    let (cols, rows) = terminal_size().unwrap(); // never use it :-p
    let pause = Duration::from_millis(1000);
    let nums = -5_i64..=5_i64;
    let zoom = 3;
    let wrld_gen: WorldGen = create_world_gen(cols, rows, zoom);
    println!("{}{}", clear::All, style::Reset);

    for i in nums {
        //only refresh in steps == z
        let grid1 = wrld_gen(i, i);
        let grid2 = redist_with_zoom(grid1, (cols, rows), (0, 0), zoom); // clever offset to move just 1 col/row

        print!("{}", cursor::Goto(1, 1));
        grid2
            .into_iter()
            .map(colorize_row)
            .for_each(|r| print!("{}", r));
        sleep(pause);
    }
    println!("{}", style::Reset);
}
