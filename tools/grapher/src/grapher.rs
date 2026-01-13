use std::{cell::RefCell, collections::HashMap};

use ieee1800_2023_ast::ast::{AST, Expression, Item, Sequence};
use petgraph::{Graph, graph::{DiGraph, NodeIndex}};

use crate::{config::Config, recursion::Checker};







pub struct Grapher {
    ast_ : AST,
    config_ : Config,
    recursion_checker_ : RefCell<Checker>,
}


impl Grapher {
    pub fn new(ast : AST, config : Config) -> Self {
        Grapher {  
            ast_ : ast,
            config_ : config,
            recursion_checker_ : Checker::new().into(),
        }
    }


    pub fn create_graph(&self) -> Graph<String, u32> {
        let entry_point = self.config_.entry_point.clone();
        let mut graph = DiGraph::<String, u32>::new();

        let node = graph.add_node("START".to_string());

        self.recursion_checker_.borrow_mut().check(entry_point.clone());
        self.add_rule_name_nodes(&mut graph, vec![node], entry_point);
        println!("Done creating graph");

        graph
    }

    pub fn add_rule_name_nodes(&self, graph : &mut Graph<String, u32>, parent_nodes : Vec<NodeIndex>,   rule_name : String) -> Vec<NodeIndex> {
        if let Some(node) = self.recursion_checker_.borrow_mut().check(rule_name.clone()) {
            for parent_node in &parent_nodes {
                graph.add_edge(*parent_node, node, 5);
                return vec![];
            }
        }
        

        let node = graph.add_node(rule_name.clone());
        self.recursion_checker_.borrow_mut().add_expression_node(rule_name.clone(), node.clone());

        for parent_node in parent_nodes {
            graph.add_edge(parent_node, node, 5);
        }

        let expression = self.ast_.get_rule(&rule_name).expect("Rule not found");
        let nodes = self.add_expression_nodes(graph, vec![node], expression);


        self.recursion_checker_.borrow_mut().pop();

//        println!("-- DONE WITH RULE --");

        nodes
        

    }

    pub fn add_expression_nodes(&self, graph : &mut Graph<String,u32>, parent_nodes : Vec<NodeIndex>, expression :&Expression) -> Vec<NodeIndex> {
        let mut leaf_nodes = Vec::new();
        for seq in &expression.sequences {
            leaf_nodes.extend(self.add_sequence_nodes(graph,  parent_nodes.clone(), seq));
        }
        leaf_nodes
    }


    pub fn add_sequence_nodes(&self, graph: &mut Graph<String, u32>, mut parent_nodes : Vec<NodeIndex>, sequence: &Sequence) -> Vec<NodeIndex> {
        for item in &sequence.items {
            parent_nodes = self.add_item_nodes(graph, parent_nodes, item);
        }
        parent_nodes
    }

    pub fn add_item_nodes(&self, graph: &mut Graph<String, u32>, parent_nodes : Vec<NodeIndex>, item : &Item) -> Vec<NodeIndex> {
        match item {
            Item::Literal(literal) => {
                vec![self.add_node(graph, format!("L({})", literal), parent_nodes)]
            },
            Item::RegEx(regex) => {
                vec![self.add_node(graph, format!("RE({})", regex), parent_nodes)]
            },

            Item::Optional(expression) => {
                 self.add_optional_node(graph, expression, parent_nodes)
            },
            Item::Repetition(expression) => {
                vec![self.add_node(graph, format!("REPETITION"), parent_nodes)]
                // self.add_repetition_node(graph, expression, parent_nodes)
            }
            Item::RuleName(rule_name) => {
                let nodes = self.add_rule_name_nodes(graph, parent_nodes, rule_name.clone());
                nodes
            },
        }
    }


    pub fn add_node(&self, graph : &mut Graph<String, u32>, label : String, parent_nodes : Vec<NodeIndex>) -> NodeIndex {
        let node = graph.add_node(label);
        for parent_node in parent_nodes {
            graph.add_edge(parent_node, node, 5);
        }
        node
    }


    pub fn add_optional_node(&self, graph : &mut Graph<String, u32>, expression : &Expression, parent_nodes : Vec<NodeIndex>) -> Vec<NodeIndex> {
        let mut nodes = self.add_expression_nodes(graph, parent_nodes.clone(), expression);
        nodes.extend(parent_nodes);
        nodes
    }

    pub fn add_repetition_node(&self, graph : &mut Graph<String, u32>, expression : &Expression, parent_nodes : Vec<NodeIndex>) -> Vec<NodeIndex> {
        let mut nodes = self.add_expression_nodes(graph, parent_nodes.clone(), expression);
        nodes.extend(parent_nodes);
        nodes
    }

}


