use image::{DynamicImage, GenericImageView, Pixel, Rgb};
use rgb2ansi256::rgb_to_ansi256;
use std::io::{self, Write};

macro_rules! fg {
    ($color:expr,$c:expr) => {
        format!("\x1b[38;5;{}m{}", $color, $c)
    };
}

pub trait ToText {
    fn text(&self, palette: &[u8], w: &mut impl Write) -> io::Result<()>;
}
impl ToText for DynamicImage {
    fn text(&self, palette: &[u8], w: &mut impl Write) -> io::Result<()> {
        let p_len = (palette.len() - 1) as f32;
        let height = (self.height() / 2) as usize;
        for mut y in 0..height {
            y <<= 1;
            let mut last_p: Option<u8> = None;
            for x in 0..self.width() {
                let p = unsafe { self.unsafe_get_pixel(x, y as u32) };
                let y = p.to_luma()[0] as f32 / 255_f32;
                let a = i2range(p[3]);
                let pos = (p_len * y).round();
                let i: usize = (pos * a).round() as usize;
                let ch = palette[i] as char;
                if y <= 0.01 {
                    write!(w, "{ch}")?;
                    continue;
                }
                // must round it into ansi256 or we will get duplicates
                let color = p.to_rgb().ansi_256();
                if let Some(last) = last_p {
                    if last == color {
                        write!(w, "{ch}")?;
                        continue;
                    }
                }

                last_p = Some(color);
                write!(w, "{}", fg!(color, ch))?;
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

fn i2range(i: u8) -> f32 {
    (i as f32 / u8::MAX as f32).clamp(0.0, 1.0)
}
