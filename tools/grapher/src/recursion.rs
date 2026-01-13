use std::{collections::HashMap};

use petgraph::graph::NodeIndex;

pub struct Checker {
    visiting_path: Vec<String>,
    expression_map: HashMap<String,NodeIndex>
}


impl Checker{
    pub fn new() -> Self {
        Checker { 
            visiting_path: Vec::new(),
            expression_map : HashMap::new()
        }
    }

    pub fn check(&mut self, rule : String) -> Option<NodeIndex> {
        if self.visiting_path.contains(&rule) {
            println!("Recursion: {} > {}", self.visiting_path.join(" > "), rule);
            let result = self.expression_map.get(&rule).unwrap().clone();

            return Some(result)
        }
        None
    }

    pub fn add_expression_node(&mut self, rule : String, expression_node : NodeIndex) {
        self.visiting_path.push(rule.clone());
        self.expression_map.insert(rule, expression_node);

        println!("Visiting: {}", self.visiting_path.join(" > "));
    }

    pub fn pop(&mut self) {
        let rulename = self.visiting_path.pop().unwrap();
        self.expression_map.remove(&rulename);

        println!("POP: {}", self.visiting_path.join(" > "));
    }


    
}
