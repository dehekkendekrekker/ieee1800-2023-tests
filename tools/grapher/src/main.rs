use std::{fs, path::PathBuf};
use anyhow::Result;
use clap::Parser;

use grapher::{config::Config, grapher::RailRoadConverter, html_grapher::HtmlRailroadGenerator, paths::PathFinder};
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
    let has_entry_point = config.entry_point.is_some();

    let path_finder = PathFinder::new(ast, config);

    let pathmap = path_finder.build_pathmap();

//    let path_generator = PathGenerator::from(pathmap);
//    let paths = path_generator.generate_all(10, 4);


    // Determine output format from file extension
    let extension = cli.output.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("svg");

    let output_content = match extension {
        "html" | "htm" => {
            // Generate interactive HTML
            let generator = HtmlRailroadGenerator::new(pathmap);
            if has_entry_point {
                generator.generate_html()
            } else {
                generator.generate_html_all_rules()
            }
        }
        _ => {
            // Default to SVG
            let converter = RailRoadConverter::new(pathmap, cli.expand_depth);
            let diagram = converter.generate_diagram();
            diagram.to_string()
        }
    };

    fs::write(cli.output, output_content)?;

    Ok(())

}
