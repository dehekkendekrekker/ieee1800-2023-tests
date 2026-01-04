use std::{fs, path::PathBuf, process};

use petgraph::graph::DiGraph;
use clap::Parser;


mod graphml;
mod parser;

#[derive(Parser, Debug)]
#[command(author = "DHK")]
#[command(name = "grapher")]
#[command(version = "1.0")]
#[command(about = "Converts IEEE1800-2023 BNF to GraphML")]
#[command(author, long_about = None)]
struct Cli {
    #[arg(long, value_name = "FILE")]
    input: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let text = match fs::read_to_string(&cli.input) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", cli.input.display(), e);
            process::exit(1);
        }
    };


    let mut graph = DiGraph::<&str, i32>::new();

    // Add nodes
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");

    // Add edges (node indices, edge weight)
    graph.add_edge(a, b, 10);
    graph.add_edge(b, c, 10);

    println!("{}", graphml::to_graphml(&graph));
}
