use std::{fs, path::PathBuf};
use anyhow::Result

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


    // Derive configuration from config file
    let config = Config::load(cli.config)?;


    // Load ast
    let ast : AST = serde_json::from_reader(
        fs::File::open(cli.ast).expect("Error opening input file")
    ).expect("Error deserialzing json");


    // Instantiate grapher
    let grapher = Grapher::new(ast);

//    let graph = grapher.create_graph(cli.entry);


//    graph.node_count();
//
//






    
/*

    let mut graph = DiGraph::<&str, i32>::new();

    // Add nodes
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");

    // Add edges (node indices, edge weight)
    graph.add_edge(a, b, 10);
    graph.add_edge(b, c, 10);
*/
//    println!("{}", to_graphml(&graph));

    fs::write(cli.output, to_graphml(&graph))?;

    Ok(())

}
