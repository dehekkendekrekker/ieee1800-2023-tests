use std::{cell::RefCell, collections::HashMap};

use ieee1800_2023_ast::ast::{AST, Expression, Item, Sequence};
use petgraph::{Graph, graph::{DiGraph, NodeIndex}};

use crate::{config::Config, recursion::Checker};







pub struct Grapher {
    graph_ : Graph<String, u32>,
    ast_ : AST,
    config_ : Config,
    recursion_checker_ : RefCell<Checker>,
    node_map_ : HashMap<String, NodeIndex>,
}


impl Grapher {
    pub fn new(ast : AST, config : Config) -> Self {
        Grapher {  
            graph_ : DiGraph::<String, u32>::new(),
            ast_ : ast,
            config_ : config,
            recursion_checker_ : Checker::new().into(),
            node_map_ : HashMap::new()
        }
    }

    pub fn create_graph(mut self) -> Graph<String, u32> {
        let entry_point = self.config_.entry_point.clone();

        let node = self.graph_.add_node("START".to_string());

        self.recursion_checker_.borrow_mut().check(entry_point.clone());
        self.add_rule_name_nodes( vec![node], entry_point);
        println!("Done creating graph");

        self.graph_
    }

    fn add_rule_name_nodes(&mut self, parent_nodes : Vec<NodeIndex>,   rule_name : String) -> Vec<NodeIndex> {
        // Check for recursion
        if let Some(node) = self.recursion_checker_.borrow_mut().check(rule_name.clone()) {
            for parent_node in &parent_nodes {
                self.graph_.add_edge(*parent_node, node, 5);
                return vec![];
            }
        }

        
        // Create the node for the expression using the rule's name
        let node = self.graph_.add_node(rule_name.clone());
        self.recursion_checker_.borrow_mut().add_expression_node(rule_name.clone(), node.clone());

        for parent_node in parent_nodes {
            self.graph_.add_edge(parent_node, node, 5);
        }

        if self.config_.ignore_rules.contains(&rule_name) {
            return vec![node];
        }

        let expression = self.ast_.get_rule(&rule_name).expect("Rule not found");
        let nodes = self.add_expression_nodes(vec![node], expression);


        self.recursion_checker_.borrow_mut().pop();

//        println!("-- DONE WITH RULE --");

        nodes
        

    }

    pub fn add_expression_nodes(&mut self, parent_nodes : Vec<NodeIndex>, expression :Expression) -> Vec<NodeIndex> {
        let mut leaf_nodes = Vec::new();
        for seq in &expression.sequences {
            leaf_nodes.extend(self.add_sequence_nodes(parent_nodes.clone(), seq));
        }
        leaf_nodes
    }


    fn add_sequence_nodes(&mut self, mut parent_nodes : Vec<NodeIndex>, sequence: &Sequence) -> Vec<NodeIndex> {
        for item in sequence.items.clone() {
            parent_nodes = self.add_item_nodes(parent_nodes, item);
        }
        parent_nodes
    }

    fn add_item_nodes(&mut self, parent_nodes : Vec<NodeIndex>, item : Item) -> Vec<NodeIndex> {
        match item {
            Item::Literal(literal) => {
                vec![self.add_node(format!("L({})", literal), parent_nodes)]
            },
            Item::RegEx(regex) => {
                vec![self.add_node(format!("RE({})", regex), parent_nodes)]
            },

            Item::Optional(expression) => {
                 self.add_optional_node(*expression, parent_nodes)
            },
            Item::Repetition(expression) => {
//                vec![self.add_node(graph, format!("REPETITION"), parent_nodes)]
                self.add_repetition_node(*expression, parent_nodes)
            }
            Item::RuleName(rule_name) => {
                let nodes = self.add_rule_name_nodes(parent_nodes, rule_name.clone());
                nodes
            },
        }
    }


    fn add_node(&mut self, label : String, parent_nodes : Vec<NodeIndex>) -> NodeIndex {
        let node = self.graph_.add_node(label);
        for parent_node in parent_nodes {
            self.graph_.add_edge(parent_node, node, 5);
        }
        node
    }


    pub fn add_optional_node(&mut self, expression : Expression, parent_nodes : Vec<NodeIndex>) -> Vec<NodeIndex> {
        let mut nodes = self.add_expression_nodes(parent_nodes.clone(), expression);
        nodes.extend(parent_nodes);
        nodes
    }

    pub fn add_repetition_node(&mut self, expression : Expression, parent_nodes : Vec<NodeIndex>) -> Vec<NodeIndex> {
        let mut nodes = self.add_expression_nodes(parent_nodes.clone(), expression);
        nodes.extend(parent_nodes);
        nodes
    }

/*
    fn add_named_node(&self, ) -> NodeIndex {

    }
*/
}


