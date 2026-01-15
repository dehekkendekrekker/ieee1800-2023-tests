use std::{fs, path::PathBuf};
use anyhow::Result;

use grapher::{config::Config, paths::{PathFinder, PathGenerator}};
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

    let path_finder = PathFinder::new(ast, config);
    
    let pathmap = path_finder.build_pathmap();

    let path_generator = PathGenerator::from(pathmap);
    let paths = path_generator.generate_all(10, 4);

    println!("Pathcount: {}", paths.len());
    

//    println!("Paths: {:#?}", paths);

 //   let grapher = Grapher::from(pathmap);

//    let graph = grapher.create_graph();


    /*
    // Instantiate grapher
    let grapher = Grapher::new(ast, config);

    // Create graph
    let graph = grapher.create_graph();


    println!("Node count: {}", graph.node_count());
*/

    // Write graph to FS
//    fs::write(cli.output, to_graphml(&graph))?;

    Ok(())

}
