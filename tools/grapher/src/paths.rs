use std::{collections::HashMap};

use ieee1800_2023_ast::ast::{AST, Expression, Item, Sequence};

use crate::config::Config;


#[derive(Debug, Clone)]
pub enum Node {
    Literal(String),
    RegEx(String),
    Rule(String),           // Just a reference
    Alternative(Vec<Vec<Node>>),     // Each inner Vec is an alternative
    Optional(Box<Node>),    // 0 or 1 times
    Repetition(Box<Node>),  // 0..n times
}

#[derive(Debug, Clone)]
pub struct PathMap {
    reg_ : HashMap<String, Node>,
    entry_point_ : String,
}

impl PathMap {
    pub fn new() -> Self {
        PathMap {
            reg_ : HashMap::new(),
            entry_point_: String::new(),
        }
    }

    pub fn get_rule_names(&self) -> Vec<String> {
        self.reg_.keys().cloned().collect()
    }

    pub fn add_rule(&mut self, name : &String, node : Node) {
        self.reg_.insert(name.clone(), node);
    }

    pub fn has_rule(&self, k : &String) -> bool {
        self.reg_.contains_key(k)
    }

    pub fn get_rule(&self, k : String) -> Node {
        self.reg_.get(&k).unwrap().clone()
    }

    pub fn get_rules(&self) -> HashMap<String, Node> {
        self.reg_.clone()
    }

    pub fn get_entry_point(&self) -> String {
        self.entry_point_.clone()
    }
}



pub struct PathFinder {
    ast_ : AST,
    config_ : Config,
    map_ : PathMap,
}

impl PathFinder  {
    pub fn new(ast : AST, config : Config) -> Self {
        PathFinder {
            ast_: ast,
            config_ : config,
            map_ : PathMap::new(),
        }
    }

    pub fn build_pathmap(mut self) -> PathMap{
        let entry_point = self.config_.entry_point.clone();

        self.map_.entry_point_ = entry_point.clone();
        self.add_rule_name_nodes(&entry_point);


        println!("PATHMAP: {:#?}", self.map_);

        self.map_
    }

    fn add_rule_name_nodes(&mut self, rule_name : &String)  {
        if self.map_.has_rule(rule_name) {
            return;
        }


    
        self.map_.add_rule(rule_name, Node::Literal("Placeholder".to_string()));


        let expression = self.ast_.get_rule(&rule_name).expect("Rule not found");
        let or_node = self.add_expression_nodes(&Box::new(expression));
        self.map_.add_rule(rule_name, or_node);

    }


    pub fn add_expression_nodes(&mut self, expression : &Box<Expression>) -> Node {
        let mut nodes = Vec::new();
        for sequence in &expression.sequences {
            nodes.push(self.add_sequence_nodes(sequence));
        }

        Node::Alternative(nodes)
    }

    fn add_sequence_nodes(&mut self, sequence : &Sequence) -> Vec<Node> {
        let mut item_vecs = Vec::new();

        for item in &sequence.items {
            item_vecs.push(self.add_item_node(item));
        }

        item_vecs
    }

    fn add_item_node(&mut self, item : &Item) -> Node {
        match item {
            Item::Literal(literal) => {
                Node::Literal(format!("L({})", literal))
            }
            Item::RegEx(regex) => {
                Node::RegEx(format!("RE({})", regex))
            },

            Item::Optional(expression) => {
                Node::Optional(Box::new(self.add_expression_nodes(expression)))
            },
            Item::Repetition(expression) => {
                Node::Repetition(Box::new(self.add_expression_nodes(expression)))
            }
            Item::RuleName(rule_name) => {
                self.add_rule_name_nodes(rule_name);
                Node::Rule(rule_name.clone())
            },
        }
    }
    
}



pub struct PathGenerator {
    pathmap_ : PathMap
}


impl From<PathMap> for PathGenerator {
    fn from(value: PathMap) -> Self {
        PathGenerator { pathmap_: value }
    }
}

impl PathGenerator {
    pub fn generate_all(&self, max_depth: usize, max_reps: usize) -> Vec<Vec<Node>> {
        let entry = self.pathmap_.get_entry_point();
        let root = self.pathmap_.get_rule(entry);
        self.expand_node(&root, 0, max_depth, max_reps)
    }
    
    fn expand_node(&self, node: &Node, depth: usize, max_depth: usize, max_reps: usize) -> Vec<Vec<Node>> {
        if depth > max_depth {
            return vec![vec![]];
        }
        
        match node {
            Node::Literal(s) => {
                vec![vec![Node::Literal(s.clone())]]
            }
            
            Node::RegEx(r) => {
                vec![vec![Node::RegEx(r.clone())]]
            }
            
            Node::Rule(name) => {
                let rule_node = self.pathmap_.get_rule(name.clone());
                self.expand_node(&rule_node, depth + 1, max_depth, max_reps)
            }
            
            Node::Alternative(alternatives) => {
                let mut all_paths = Vec::new();
                
                for sequence in alternatives {
                    let paths = self.expand_sequence(sequence, depth, max_depth, max_reps);
                    all_paths.extend(paths);
                }
                
                all_paths
            }
            
            Node::Optional(inner) => {
                let mut paths = Vec::new();
                
                // Path 1: skip it
                paths.push(vec![]);
                
                // Path 2: include it
                let inner_paths = self.expand_node(inner, depth, max_depth, max_reps);
                paths.extend(inner_paths);
                
                paths
            }
            
            Node::Repetition(inner) => {
                let inner_paths = self.expand_node(inner, depth, max_depth, max_reps);
                let mut all_paths = Vec::new();
                
                // 0 repetitions
                all_paths.push(vec![]);
                
                // 1 to max_reps repetitions
                for rep_count in 1..=max_reps {
                    let repeated = self.repeat_n_times(&inner_paths, rep_count);
                    all_paths.extend(repeated);
                }
                
                all_paths
            }
        }
    }
    
    fn expand_sequence(&self, nodes: &[Node], depth: usize, max_depth: usize, max_reps: usize) -> Vec<Vec<Node>> {
        if nodes.is_empty() {
            return vec![vec![]];
        }
        
        let mut current_paths = self.expand_node(&nodes[0], depth, max_depth, max_reps);
        
        for node in &nodes[1..] {
            let next_parts = self.expand_node(node, depth, max_depth, max_reps);
            current_paths = self.combine_all(&current_paths, &next_parts);
        }
        
        current_paths
    }
    
    fn combine_all(&self, first: &[Vec<Node>], second: &[Vec<Node>]) -> Vec<Vec<Node>> {
        let mut combined = Vec::new();
        
        for path_so_far in first {
            for next_part in second {
                let mut full_path = path_so_far.clone();
                full_path.extend_from_slice(next_part);
                combined.push(full_path);
            }
        }
        
        combined
    }
    
    fn repeat_n_times(&self, paths: &[Vec<Node>], n: usize) -> Vec<Vec<Node>> {
        let mut result = paths.to_vec();
        
        for _ in 1..n {
            result = self.combine_all(&result, paths);
        }
        
        result
    }
    
    // Convert paths to strings when needed
    pub fn paths_to_strings(&self, paths: &[Vec<Node>]) -> Vec<String> {
        paths.iter().map(|path| self.path_to_string(path)).collect()
    }
    
    fn path_to_string(&self, path: &[Node]) -> String {
        path.iter().map(|n| match n {
            Node::Literal(s) | Node::RegEx(s) => s.clone(),
            _ => String::new()
        }).collect()
    }
}   
