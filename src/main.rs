use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(value_name = "INPUT")]
    name: String,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    // println!("{:?}", args);

    let content = std::fs::read_to_string(args.name)?;

    for lines in content.lines() {
        println!("{lines}");
    }

    Ok(())
}
