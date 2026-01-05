use std::{fs, path::PathBuf};
use anyhow::anyhow;

use clap::Parser;

use bnf2pest::parser::parse_bnf;

#[derive(Parser, Debug)]
#[command(author = "Daniel Attevelt")]
#[command(name = "bnf2pest")]
#[command(version = "1.0")]
#[command(about = "Converts BNF grammar used in IEEE1800-2023 to pest format")]
#[command(author, long_about = None)]
struct Cli {
    #[arg(long, value_name = "FILE")]
    input: PathBuf,
}


fn main() -> Result<(), anyhow::Error>{
    let cli = Cli::parse();

    let path = cli.input.into_os_string();

    let input = fs::read_to_string(path.clone())
        .map_err(|e| anyhow!("Failed to read bnf file '{:#?}: {}", path, e))
        .unwrap();

    let ast = parse_bnf(input)
        .map_err(|e| anyhow!("Failed to parse BNF: {}", e))?;


    println!("{:#?}", ast);




    Ok(())
}
