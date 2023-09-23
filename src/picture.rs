use image::{DynamicImage, GenericImageView, Pixel, Rgb};
use rgb2ansi256::rgb_to_ansi256;
use std::io::{self, Write};

pub trait ToText {
    fn text(&self, palette: &[u8], three: bool, w: &mut impl Write) -> io::Result<()>;
}
impl ToText for DynamicImage {
    fn text(&self, palette: &[u8], three: bool, w: &mut impl Write) -> io::Result<()> {
        macro_rules! fg {
            ($color:expr,$c:expr) => {
                write!(w, "\x1b[38;5;{}m{}", $color, $c)?
            };
            (3 $color:expr,$c:expr) => {
                write!(w, "\x1b[0;34;3{}m{}", $color, $c)?
            };
        }

        let p_len = (palette.len() - 1) as f32;
        let height = (self.height() / 2) as usize;
        for mut y in 0..height {
            y <<= 1;
            let mut last_p = if three { 37 } else { 255 };
            for x in 0..self.width() {
                let p = unsafe { self.unsafe_get_pixel(x, y as u32) };
                let y = p.to_luma()[0] as f32 / 255_f32;
                let a = p[3] as f32 / u8::MAX as f32;
                let pos = (p_len * y).round();
                let i: usize = (pos * a).round() as usize;
                let ch = unsafe { *palette.get_unchecked(i) as char };
                if y <= 0.01 {
                    write!(w, "{ch}")?;
                    continue;
                }
                let color = if three {
                    p.to_rgb()
                        .0
                        .into_iter()
                        .enumerate()
                        .map(|(i, v)| (v / 127) << i)
                        .sum()
                } else {
                    p.to_rgb().ansi_256()
                };
                if last_p == color {
                    write!(w, "{ch}")?;
                    continue;
                }

                last_p = color;
                if three {
                    fg!(3 color, ch);
                } else {
                    fg!(color, ch);
                }
            }
            writeln!(w)?;
        }
        Ok(())
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
