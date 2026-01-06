use std::process::exit;

use crate::ast::{AST, Expression, Item, Sequence};
use anyhow::{Result, anyhow};


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


pub fn parse_rule(pair: Pair<Rule>) -> Result<(String, Expression)> {
    let mut name = String::new();
    let mut expression = Expression::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::rule_name => {
                name = inner.as_str().to_string();
            },
            Rule::expression => {
                expression = parse_expression(inner)?;
            }
            _ => {}
            
        }
    }

    Ok((name, expression))
}

pub fn parse_expression(rule_pair: Pair<Rule>) -> Result<Expression> {
    let mut expression = Expression::new();

    for inner in rule_pair.into_inner() {
        match inner.as_rule() {
            Rule::sequence => {
                expression.add_sequence(parse_sequence(inner)?);
            },
            _ => {}
        }
    }

    return Ok(expression)
}

pub fn parse_sequence(rule_pair: Pair<Rule>) -> Result<Sequence> {
    let mut sequence = Sequence::new();

    for inner in rule_pair.into_inner() {
        match inner.as_rule() {
            Rule::item => {
                sequence.add_item(parse_item(inner)?);
            },
            _ => {
                println!("{:#?}", inner.as_rule());
            }
        }
    }

    return Ok(sequence)
}


pub fn parse_item(rule_pair: Pair<Rule>) -> Result<Item> {
    for inner in rule_pair.into_inner() {
        match inner.as_rule() {
            Rule::literal => {
                let literal = parse_literal(inner)?;
                return Ok(Item::Literal(literal));
            },

            Rule::rule_name => {
                let name = inner.as_str().to_string();
                return Ok(Item::RuleName(name));
            },
            Rule::optional => {
                let expression_rule : Pair<Rule> = inner.into_inner()
                    .find(|p| p.as_rule() == Rule::expression)
                    .unwrap();

                let expression = parse_expression(expression_rule)?;
                return Ok(Item::Optional(Box::from(expression)));
            },
            Rule::repetition => {
                let expression_rule : Pair<Rule> = inner.into_inner()
                    .find(|p| p.as_rule() == Rule::expression)
                    .unwrap();

                let expression = parse_expression(expression_rule)?;
                return Ok(Item::Repetition(Box::from(expression)));
            },
            _ => {}
        }
    }


   Err(anyhow!("Something went wrong".to_string()))
}


pub fn parse_literal(rule_pair: Pair<Rule>) -> Result<String> {
    for inner in rule_pair.into_inner() {
        match inner.as_rule() {
            Rule::text1 |
            Rule::text2 => {
                return Ok(inner.as_str().to_string());
            },
            _ => {}
        }
    }

    Err(anyhow!("No text1 or text2 found"))
}


