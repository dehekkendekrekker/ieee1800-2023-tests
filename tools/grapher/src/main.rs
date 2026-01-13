use std::{fs, path::PathBuf, process::exit};
use anyhow::Result;

use grapher::{config::Config, grapher::Grapher, graphml::to_graphml};
use ieee1800_2023_ast::ast::AST;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(author = "DHK")]
#[command(name = "grapher")]
#[command(version = "1.0")]
#[command(about = "Converts IEEE1800-2023 AST to GraphML")]
#[command(author, long_about = None)]
struct Cli {
    #[arg(long, value_name = "FILE_NAME")]
    ast: PathBuf,

    #[arg(long, value_name = "FILE_NAME")]
    config : PathBuf,

    #[arg(long, value_name = "FILE_NAME")]
    output: PathBuf,
}

fn main() -> Result<()> {

    // Load command line arguments
    let cli = Cli::parse();


    // Load ast
    let ast : AST = serde_json::from_reader(
        fs::File::open(cli.ast).expect("Error opening input file")
    ).expect("Error deserialzing json");

    // Derive configuration from config file
    let config = Config::load(cli.config)?;

    // Instantiate grapher
    let grapher = Grapher::new(ast, config);

    let graph = grapher.create_graph();


    println!("Node count: {}", graph.node_count());


    fs::write(cli.output, to_graphml(&graph))?;

    Ok(())

}
