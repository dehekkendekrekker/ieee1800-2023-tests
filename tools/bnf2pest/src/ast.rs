#[derive(Debug,Clone)]
pub struct Definition {
    pub expression : String
}



#[derive(Debug,Clone)]
pub struct Production {
    pub name : String,
    pub definition : Definition
}



#[derive(Debug, Clone)]
pub struct Syntax {
    pub rules: Vec<Production>
}
