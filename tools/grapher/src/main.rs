use std::{fs, path::PathBuf, process};

use grapher::{grapher::Grapher, graphml::to_graphml};
use ieee1800_2023_ast::ast::AST;
use petgraph::graph::DiGraph;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(author = "DHK")]
#[command(name = "grapher")]
#[command(version = "1.0")]
#[command(about = "Converts IEEE1800-2023 AST to GraphML")]
#[command(author, long_about = None)]
struct Cli {
    #[arg(long, value_name = "FILE_NAME")]
    input: PathBuf,

    #[arg(long, value_name = "RULE_NAME")]
    entry: String,
}

fn main() {
    let cli = Cli::parse();


    let mut ast : AST = serde_json::from_reader(
        fs::File::open(cli.input).expect("Error opening input file")
    ).expect("Error deserialzing json");


    let grapher = Grapher::new(ast);

    let graph = grapher.create_graph(cli.entry);






    
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
    println!("{}", to_graphml(&graph));
}
