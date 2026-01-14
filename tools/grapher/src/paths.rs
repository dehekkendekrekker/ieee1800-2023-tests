use std::{collections::HashMap, os::unix::process::parent_id, usize};

use clap::builder::Str;
use ieee1800_2023_ast::ast::{AST, Expression, Item, Sequence};

use crate::config::Config;

use slab::Slab;

#[derive(Debug)]
pub enum Node {
    Literal(String),
    RegEx(String),
    Rule(String),
    OrOpen,
    OrClose,
    OptOpen,
    OptClose,
    RepOpen,
    RepClose,

}


#[derive(Debug)]
pub struct PathMap {
    reg_ : HashMap<String, Vec<Node>>,
    entry_point_ : String,
}

impl PathMap {
    pub fn new() -> Self {
        PathMap {
            reg_ : HashMap::new(),
            entry_point_: String::new(),
        }
    }

    pub fn add_rule(&mut self, name : String, chain : Vec<Node>) {
        self.reg_.insert(name, chain);
    }

    pub fn has_rule(&self, k : String) -> bool {
        self.reg_.contains_key(&k)
    }
}



pub struct PathFinder {
    ast_ : AST,
    config_ : Config,
    rr_ : PathMap,
}

impl PathFinder {
    pub fn new(ast : AST, config : Config) -> Self {
        PathFinder { 
            ast_: ast,
            config_ : config,
            rr_ : PathMap::new(),
        }
    }

    pub fn start(mut self) {
        let entry_point = self.config_.entry_point.clone();

        self.rr_.entry_point_ = entry_point.clone();
        self.add_rule_name_nodes(Vec::new(), entry_point);


        println!("NodeMap: {:#?}", self.rr_);

        
    }

    fn add_rule_name_nodes(&mut self, mut chain : Vec<Node>, rule_name : String) -> Vec<Node> {
        chain.push(Node::Rule(rule_name.clone()));

        if !self.rr_.has_rule(rule_name.clone()) {
            // Insert empty vec to prevent recursion
            self.rr_.add_rule(rule_name.clone(), Vec::new());
            let expression = self.ast_.get_rule(&rule_name).expect("Rule not found");
            let subchain = self.add_expression_nodes(Vec::new(), expression);

            self.rr_.add_rule(rule_name, subchain);
        }
        

        chain
    }


    pub fn add_expression_nodes(&mut self, mut chain : Vec<Node>, expression : Expression) -> Vec<Node> {
        for sequence in &expression.sequences {
            chain.push(Node::OrOpen);
            chain = self.add_sequence_nodes(chain, sequence);
            chain.push(Node::OrClose);
        }

        chain
    }

    fn add_sequence_nodes(&mut self, mut chain: Vec<Node>, sequence : &Sequence) -> Vec<Node> {
        for item in &sequence.items {
            chain = self.add_item_node(chain, item.clone());
        }

        chain
    }

    fn add_item_node(&mut self, mut chain : Vec<Node>, item : Item) -> Vec<Node> {
        match item {
            Item::Literal(literal) => {
                chain.push(Node::Literal(format!("L({})", literal)));
                chain
            }
            Item::RegEx(regex) => {
                chain.push(Node::RegEx(format!("RE({})", regex)));
                chain
            },

            Item::Optional(expression) => {
                chain.push(Node::OptOpen);
                chain = self.add_expression_nodes(chain, *expression);
                chain.push(Node::OptClose);
                chain
            },
            Item::Repetition(expression) => {
                chain.push(Node::RepOpen);
                chain = self.add_expression_nodes(chain, *expression);
                chain.push(Node::RepClose);
                chain
            }
            Item::RuleName(rule_name) => {
                chain = self.add_rule_name_nodes(chain, rule_name.clone());
                chain
            },
        }
    }
    
}







    
    
