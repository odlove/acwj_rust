use clap::Parser;
use std::io;
use std::path::PathBuf;

mod scan;
mod parser;


#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,
    input: PathBuf,
}


fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let content = std::fs::read_to_string(&cli.input)?;

    let tokens = scan::scanfile(&content);

    println!();
    let ast = parser::parse(tokens);

    Ok(())
}
