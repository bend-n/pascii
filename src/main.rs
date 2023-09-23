use clap::Parser;
use picture::ToText;
use std::{io::BufWriter, path::PathBuf};

mod picture;
#[derive(Parser)]
#[command(name = "pascii")]
#[command(bin_name = "pascii")]
/// turn image into ascii with ansi
struct Args {
    /// File to take from
    #[arg()]
    from: PathBuf,
    /// Palette of chars to use
    #[arg(long, default_value = "   ...,:clodxkO0KXM")]
    pal: String,
    /// 3 bit rgb?
    #[arg(long)]
    three: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let p = image::open(args.from)?;
    let mut s = BufWriter::new(std::io::stdout().lock());
    p.text(args.pal.as_bytes(), args.three, &mut s)?;
    Ok(())
}
