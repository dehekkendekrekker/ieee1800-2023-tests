use std::collections::HashMap;

#[derive(Debug,Clone)]
pub struct Definition {
    pub expression : String
}


#[derive(Debug, Clone)]
pub struct AST {
    pub rules : HashMap<String, Definition>
}

impl AST {
    pub fn new() -> Self {
        AST { rules: HashMap::new() }
    }

    pub fn add_rule(&mut self, name : String, definition : Definition) {
        self.rules.insert(name, definition);
    }
    
}
