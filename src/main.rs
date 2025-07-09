use anyhow::Result;
use clap::Parser;

mod sitegen;
use sitegen::SiteGen;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"), " - a static site generator, licensed under ", env!("CARGO_PKG_LICENSE")),
    long_about = None
)]
struct Args {
    /// Local root folder containing template files
    root: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut sitegen = SiteGen::new(&args.root)?;
    sitegen.run()
}
