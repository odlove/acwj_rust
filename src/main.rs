use clap::Parser;

use crate::token::Token;


#[derive(Parser, Debug)]
struct Cli {
    #[arg(value_name = "INPUT")]
    path: String,
}

mod token;
mod scanner;

use scanner::Scanner;



fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    // println!("{:?}", args);

    let content = std::fs::read_to_string(args.path)?;
    let mut scanner = Scanner::new(&content);

    let mut tok = scanner.scan();
    while tok != Token::EOF {
        println!("{tok:?}");
        tok = scanner.scan();
    }

    Ok(())
}
