use std::{cell::RefCell, collections::HashMap};

use ieee1800_2023_ast::ast::{AST, Expression, Item, Sequence};
use petgraph::{Graph, graph::{self, DiGraph, NodeIndex}};

use crate::{config::Config, recursion::Checker, paths::{Node, NodeArena}};







pub struct Grapher {
    graph_ : Graph<String, u32>,
    ast_ : AST,
    config_ : Config,
    recursion_checker_ : RefCell<Checker>,
    graph_node_map_ : HashMap<String, NodeIndex>,
    path_nodes_ : NodeArena,
    
}


impl Grapher {
    pub fn new(ast : AST, config : Config) -> Self {
        Grapher {  
            graph_ : DiGraph::<String, u32>::new(),
            ast_ : ast,
            config_ : config,
            recursion_checker_ : Checker::new().into(),
            graph_node_map_ : HashMap::new(),
            path_nodes_ : NodeArena::new(),

        }
    }

    pub fn create_graph(mut self) -> Graph<String, u32> {
        let entry_point = self.config_.entry_point.clone();

        let mut path = Node::from(self.add_cached_node("START".to_string()));

        self.recursion_checker_.borrow_mut().check(entry_point.clone());
        path = path.add_child(self.add_rule_name_nodes( entry_point));

        let idx = self.path_nodes_.add(path);
        println!("Done creating graph");


        println!("Nodes: {:#?}", self.graph_node_map_);


 
        let paths = self.path_nodes_.leaf_paths(idx);
        for path in paths {
            for window in path.windows(2) {
                let (p, c) = (window[0], window[1]);
                self.graph_.add_edge(p, c, 5);
            }

        };






//        println!("Path: {:#?}", self.path_nodes_);




        self.graph_
    }

    fn add_rule_name_nodes(&mut self, rule_name : String) -> usize {
        // Create the node for the expression using the rule's name

        let node = Node::from(self.add_cached_node(rule_name.clone()));
//        self.recursion_checker_.borrow_mut().add_expression_node(rule_name.clone(), node.clone());

//        for parent_node in parent_nodes {
//            self.graph_.add_edge(parent_node, node, 5);
//        }

//        if self.config_.ignore_rules.contains(&rule_name) {
//            return vec![node];
//        }

        let expression = self.ast_.get_rule(&rule_name).expect("Rule not found");
        let path = node.add_child(self.add_expression_nodes(expression));



//        self.recursion_checker_.borrow_mut().pop();

//        println!("-- DONE WITH RULE --");

        self.path_nodes_.add(path)
        

    }

    pub fn add_expression_nodes(&mut self, expression :Expression) -> usize {
        let mut path  = Node::new();
        for seq in &expression.sequences {
            path = path.add_child(self.add_sequence_nodes(seq));
        }
        self.path_nodes_.add(path)
    }


    fn add_sequence_nodes(&mut self, sequence: &Sequence) -> usize {
        let mut node = Node::new();
        for item in sequence.items.clone() {
            node = node.add_child(self.add_item_nodes(item));
        }
        
        self.path_nodes_.add(node)
    }

    fn add_item_nodes(&mut self, item : Item) -> usize {
        match item {
            Item::Literal(literal) => {
                let node = Node::from(self.add_cached_node(format!("L({})", literal)));
                self.path_nodes_.add(node)
            },
            Item::RegEx(regex) => {
                let node = Node::from(self.add_cached_node(format!("RE({})", regex)));
                self.path_nodes_.add(node)
            },

            Item::Optional(expression) => {
                 self.add_optional_node(*expression)
            },
            Item::Repetition(expression) => {
//                vec![self.add_node(graph, format!("REPETITION"), parent_nodes)]
                self.add_repetition_node(*expression)
            }
            Item::RuleName(rule_name) => {
                self.add_rule_name_nodes(rule_name.clone())
            },
        }
    }



    pub fn add_optional_node(&mut self, expression : Expression) -> usize {
        let mut node = Node::new();
        node = node.add_child(self.add_expression_nodes(expression));
        node = node.add_child(self.path_nodes_.add(Node::new()));
        self.path_nodes_.add(node)
    }

    pub fn add_repetition_node(&mut self, expression : Expression) -> usize {
        // Treated as included once for now
       self.add_expression_nodes(expression)
//        path.add_child(Path::new())
    }


    pub fn add_cached_node(&mut self, name : String) -> NodeIndex {
        match self.graph_node_map_.get(&name) {
            Some(node) => { node.clone() },
            None => {
                let node = self.graph_.add_node(name.clone());
                self.graph_node_map_.insert(name, node.clone());
                node
            }
        }
    }

/*
    fn add_named_node(&self, ) -> NodeIndex {

    }
*/
}


