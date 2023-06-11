use image::{DynamicImage, GenericImageView, Pixel, Rgb};
#[cfg(rayon)]
use rayon::prelude::*;
use rgb2ansi256::rgb_to_ansi256;

macro_rules! fg {
    ($color:expr,$c:expr) => {
        format!("\x1b[38;5;{}m{}", $color, $c)
    };
}

pub trait ToText {
    fn text(&self, palette: &[u8]) -> String;
}
impl ToText for DynamicImage {
    fn text(&self, palette: &[u8]) -> String {
        let p_len = (palette.len() - 1) as f32;
        let mut output: Vec<String> = vec![];
        let height = (self.height() / 2) as usize;
        output.resize(height, String::new());
        #[cfg(not(rayon))]
        let iter = output.iter_mut();
        #[cfg(rayon)]
        let iter = output.par_iter_mut();
        iter.enumerate().for_each(|(mut y, output)| {
            y <<= 1;
            let mut last_p: Option<u8> = None;
            for x in 0..self.width() {
                let p = unsafe { self.unsafe_get_pixel(x, y as u32) };
                let y = p.to_luma()[0] as f32 / 255_f32;
                let a = i2range(p[3]);
                let pos = (p_len * y).round();
                let i: usize = (pos * a).round() as usize;
                let ch = palette[i] as char;
                // must round it into ansi256 or we will get duplicates
                let color = p.to_rgb().ansi_256();
                if let Some(last) = last_p {
                    if last == color {
                        output.push(ch);
                        continue;
                    }
                }
                last_p = Some(color);
                output.push_str(&fg!(color, ch));
            }
        });
        output.join("\n")
    }
}

trait Ansi256 {
    fn ansi_256(&self) -> u8;
}

impl Ansi256 for Rgb<u8> {
    fn ansi_256(&self) -> u8 {
        rgb_to_ansi256(self[0], self[1], self[2])
    }
}

fn i2range(i: u8) -> f32 {
    (i as f32 / u8::MAX as f32).clamp(0.0, 1.0)
}

#[test]
fn ansi_color() {
    macro_rules! test {
        ($c:expr, $expect:expr) => {{
            let args: [u8; 3] = $c;
            assert_eq!(Rgb::from(args).ansi_256(), $expect)
        }};
    }
    test!([0, 0, 255], 21);
    test!([0, 255, 0], 46);
    test!([255, 0, 0], 196);
}
