pub mod math {
    pub fn fraction(i: u64) -> f64 {
        let max = u64::MAX as f64;
        (i as f64 % max) / max
    }

    pub fn div_ceil(a: u16, b: u16) -> u16 {
        (a as f64 / b as f64).ceil() as u16
    }
}

// TODO make a similar module for Ascii
#[allow(dead_code)]
pub mod colors {
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn div() {
        assert_eq!(math::div_ceil(1, 3), 1);
        assert_eq!(math::div_ceil(1, 100), 1);
        assert_eq!(math::div_ceil(3, 4), 1);
        assert_eq!(math::div_ceil(4, 3), 2);
    }

    #[test]
    fn frac() {
        let m = u64::MAX;
        let xs = [m / 2, m / 4, m / 666, 1_000_000, 1_000, 1];
        let mut prev = m as f64;
        for n in xs {
            let f = math::fraction(n);
            assert!(f < 1.0000000);
            assert!(f > 0.0000000);
            assert!(f < prev);
            prev = f;
        }
    }
}

type TermSize = (u16, u16);
type Offset = (i16, i16);

pub fn redist_with_zoom<T: Copy>(
    grid: Vec<Vec<T>>,
    (w, h): TermSize,
    (xo, yo): Offset,
    z: u16,
) -> Vec<Vec<T>> {
    (0..h)
        .map(|y| {
            (0..w)
                .map(|x| {
                    // på trynet!
                    let ypos = (y / z) as i16 + 1 + yo;
                    let xpos = (x / z) as i16 + 1 + xo;
                    assert!(ypos >= 0);
                    assert!(xpos >= 0);
                    grid[ypos as usize][xpos as usize]
                })
                .collect()
        })
        .collect()
}
