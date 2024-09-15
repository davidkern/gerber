use clap::Parser;
use gerber::GerberLayer;
use std::fs::read_to_string;

#[derive(Parser)]
struct Cli {
    /// Name of the file to dump
    filename: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let src = read_to_string(cli.filename)?;
    let layer = GerberLayer::parse(&src)?;

    println!("{:?}", layer);

    Ok(())
}
