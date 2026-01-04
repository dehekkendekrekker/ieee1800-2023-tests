use crate::ast::{Definition, Production, Syntax};
use anyhow::Result;


use pest::{Parser, iterators::Pair};
//use pest::iterators::{Pair, Pairs};
//use pest::pratt_parser::{Assoc, Op, PrattParser};
//use anyhow::{Result, anyhow};
//use crate::ast::*;

#[derive(pest_derive::Parser)]
#[grammar = "bnf.pest"]
pub struct BNFParser;






pub fn parse_bnf(input: String) -> Result<Syntax> {
    let mut pairs = BNFParser::parse(Rule::syntax, input.as_str())?;
    let syntax_pair = pairs.next().unwrap();


    let rule_pairs = syntax_pair.into_inner();
    println!("Rules: {}", rule_pairs.len());
    
    let mut productions = Vec::new();
    for rule_pair in rule_pairs{
        if rule_pair.as_rule() == Rule::rule {
            productions.push(parse_rule(rule_pair)?);
        }
    }

    Ok(Syntax { rules: productions })
}


pub fn parse_rule(pair: Pair<Rule>) -> Result<Production> {
    let mut name = String::new();

    for inner in pair.into_inner() {
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

    Ok(Production { name: name , 
        definition: def })
}
