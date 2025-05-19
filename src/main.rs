use clap::Parser;
use std::io;
use std::path::PathBuf;

mod scan;


#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,
    input: PathBuf,
}


fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let content = std::fs::read_to_string(&cli.input)?;

    scan::scanfile(&content);

    Ok(())
}
