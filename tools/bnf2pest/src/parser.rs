use std::collections::HashMap;

use crate::ast::{Definition, AST};
use anyhow::Result;


use pest::{Parser, iterators::Pair};
//use pest::iterators::{Pair, Pairs};
//use pest::pratt_parser::{Assoc, Op, PrattParser};
//use anyhow::{Result, anyhow};
//use crate::ast::*;

#[derive(pest_derive::Parser)]
#[grammar = "bnf.pest"]
pub struct BNFParser;






pub fn parse_bnf(input: String) -> Result<AST> {
    let mut parsed_bnf = BNFParser::parse(Rule::syntax, input.as_str())?;
    
    let syntax = parsed_bnf.next().unwrap();
    let rules = syntax.into_inner();

    let mut ast = AST::new();
    
    for rule in rules {
        let (name, definition) = parse_rule(rule)?;

        ast.add_rule(name, definition);

    }

    Ok(ast)

}


pub fn parse_rule(pair: Pair<Rule>) -> Result<(String, Definition)> {
    let mut name = String::new();

    for inner in pair.into_inner() {
//        println!("inner: {:#?}", inner);
        match inner.as_rule() {
            Rule::rule_name => {
                name = inner.as_str().to_string();
            },
            _ => {}
            
        }
    }

    let def = Definition {
        expression: format!("LALA")
    };

    Ok((name, def))
}
