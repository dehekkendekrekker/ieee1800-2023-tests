use std::collections::HashMap;
use serde::{Serialize, Deserialize};



#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct Expression {
    pub sequences: Vec<Sequence>,
}

impl Expression {
    pub fn new() -> Self {
        Expression{
            sequences : Vec::new()
        }
    }

    pub fn add_sequence(&mut self, sequence : Sequence) {
        self.sequences.push(sequence);
    }

    pub fn find_rule_names(&self) -> Vec<&String> {
        let mut names = Vec::new();
        for seq in &self.sequences {
            for item in &seq.items {
                item.collect_rule_names(&mut names);
            }
        }
        names
    }
}

#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct Sequence {
    pub items: Vec<Item>
}

impl Sequence {
    pub fn new() -> Self {
        Sequence { items: Vec::new() }
    }

    pub fn add_item(&mut self, item : Item) {
        self.items.push(item);
    }
}




#[derive(Debug,Serialize, Deserialize, Clone)]
pub enum Item {
    RegEx(String),
    Literal(String),
    RuleName(String),
    Optional(Box<Expression>),
    Repetition(Box<Expression>)
}


impl Item {
    fn collect_rule_names<'a>(&'a self, names: &mut Vec<&'a String>) {
        match self {
            Item::RuleName(name) => names.push(name),
            Item::Optional(expr) | Item::Repetition(expr) => {
                names.extend(expr.find_rule_names());
            }
            _ => {}
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AST {
    pub rules : HashMap<String, Expression>
}

impl AST {
    pub fn new() -> Self {
        AST { rules: HashMap::new() }
    }

    pub fn add_rule(&mut self, name : String, expression : Expression) {
        self.rules.insert(name, expression);
    }

    pub fn get_rule(&self, name : &String) -> Option<Expression> {
        self.rules.get(name).cloned()
    }

    pub fn get_referenced_rule_names(&self) -> Vec<String> {
        let mut names : Vec<String> = Vec::new();
        self.rules.values().for_each(
            |e|  e.find_rule_names().iter().for_each(|f| names.push(f.to_string()))
        );

        names.sort();
        names.dedup();
        names

    }
    
}
