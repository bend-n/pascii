use clap::Parser;
use picture::ToText;
use std::{io::BufWriter, path::PathBuf};

mod picture;
#[derive(Parser)]
#[command(name = "pascii")]
#[command(bin_name = "pascii")]
struct Args {
    #[arg()]
    from: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let p = image::open(args.from)?;
    let mut s = BufWriter::new(std::io::stdout().lock());
    p.text(b"   ...,:clodxkO0KXM", &mut s)?;
    Ok(())
}
