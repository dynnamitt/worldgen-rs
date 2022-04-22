pub fn fraction(i: u64) -> f64 {
    let max = u64::MAX as f64;
    i as f64 % max / max
}

pub fn div_ceil(a: u16, b: u16) -> u16 {
    (a as f64 / b as f64).ceil() as u16
}

pub fn redist_with_zoom<T: Copy>(grid: Vec<Vec<T>>, (w, h): (u16, u16), z: u16) -> Vec<Vec<T>> {
    if z == 1 {
        grid
    } else {
        (0..h)
            .into_iter()
            .map(|y| {
                (0..w)
                    .into_iter()
                    .map(|x| grid[(y / z) as usize][(x / z) as usize])
                    .collect()
            })
            .collect()
    }
}
