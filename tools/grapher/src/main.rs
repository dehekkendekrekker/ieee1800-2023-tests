use std::{fs, path::PathBuf};
use anyhow::Result;
use clap::Parser;

use grapher::{config::Config, grapher::RailRoadConverter, paths::PathFinder};
use ieee1800_2023_ast::ast::AST;


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

    /// Depth to expand rules inline (0 = no expansion, rules shown as terminals)
    #[arg(long, default_value_t = 0)]
    expand_depth: usize,
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

    let path_finder = PathFinder::new(ast, config);
    
    let pathmap = path_finder.build_pathmap();

//    let path_generator = PathGenerator::from(pathmap);
//    let paths = path_generator.generate_all(10, 4);


    let converter = RailRoadConverter::new(pathmap, cli.expand_depth);

    let diagram = converter.generate_diagram();

    fs::write(cli.output, diagram.to_string())?;

    Ok(())

}
