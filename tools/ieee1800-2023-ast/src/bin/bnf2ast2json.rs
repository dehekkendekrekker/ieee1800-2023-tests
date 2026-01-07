use std::{collections::HashSet, fs, path::PathBuf};
use anyhow::anyhow;

use clap::Parser;
use ieee1800_2023_ast::parser::parse_bnf;



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

/*
    let mut correct_rule_names : Vec<String> = ast.rules.keys().cloned().collect();
    correct_rule_names.sort();
    let mut referenced_rules_names = ast.get_referenced_rule_names();
    referenced_rules_names.sort();


    let set_a: HashSet<_> = correct_rule_names.iter().collect();
    let set_b: HashSet<_> = referenced_rules_names.iter().collect();
    let mut diff: Vec<_> = set_a.symmetric_difference(&set_b).collect();

    diff.sort();

    println!("Correct: ({}), {:#?}", correct_rule_names.len(), correct_rule_names);
    println!("Referenced: ({}) {:#?}", referenced_rules_names.len(), referenced_rules_names);
    println!("Diff: ({}) {:#?}", diff.len(), diff);


    println!("AST: {:#?}", ast);
*/
    let json = serde_json::to_string(&ast).unwrap();
    println!("{}", json);


    




    Ok(())
}
