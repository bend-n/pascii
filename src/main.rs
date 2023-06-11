use clap::Parser;
use picture::*;
use std::path::PathBuf;

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
    println!("{}", p.text("   ...,:clodxkO0KXM".as_bytes()));
    Ok(())
}
