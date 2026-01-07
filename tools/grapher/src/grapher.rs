use ieee1800_2023_ast::ast::{AST, Expression, Item, Sequence};
use petgraph::{Graph, graph::{DiGraph, NodeIndex}};

pub struct Grapher {
    ast_ : AST
}


impl Grapher {
    pub fn new(ast : AST) -> Self {
        Grapher {  
            ast_ : ast
        }
    }


    pub fn create_graph(&self, rule_name : String) -> Graph<String, u8> {
        let mut graph = DiGraph::<String, u8>::new();


        let node = graph.add_node("START".to_string());


        self.add_rule_name_nodes(&mut graph, node, rule_name);



        graph
    }

    pub fn add_rule_name_nodes(&self, graph : &mut Graph<String, u8>, parent_node :NodeIndex,  rule_name : String) -> Vec<NodeIndex> {
        let expression = self.ast_.get_rule(&rule_name).expect("Rule not found");
//        let rule_name_node = graph.add_node(format!("R({})", rule_name));
//        graph.add_edge(parent_node, rule_name_node, 5);

        self.add_expression_nodes(graph, parent_node, expression)

//        graph.add_edge(rule_name_node, expression_node, 5);

//        rule_name_node
    }

    pub fn add_expression_nodes(&self, graph : &mut Graph<String,u8>, parent_node: NodeIndex, expression :&Expression) -> Vec<NodeIndex> {
        let mut node_indices = Vec::new();
        for seq in &expression.sequences {
            node_indices.extend(self.add_sequence_nodes(graph, parent_node, seq));
        }
        node_indices
    }


    pub fn add_sequence_nodes(&self, graph: &mut Graph<String, u8>, from_node: NodeIndex, sequence: &Sequence) -> Vec<NodeIndex> {
        let mut from_nodes = vec![from_node];
        
        for item in &sequence.items {
            let to_nodes = match item {
                Item::Literal(text) => {
                    vec![graph.add_node(format!("L({})", text))]
                }
                Item::RuleName(rule_name) => {
                    self.add_rule_name_nodes(graph, from_node, rule_name.clone())
                }
                _ => vec![graph.add_node("DUMMY".to_string())]
            };
            
            for &from in &from_nodes {
                for &to in &to_nodes {
                    graph.add_edge(from, to, 5);
                }
            }
            
            from_nodes = to_nodes;
        }
        
        from_nodes
    }

    /*
    pub fn add_item_nodes(&self, graph : &mut Graph<String,u8>, item : &Item) -> NodeIndex {
        match item {
            Item::Literal(x) => {
                graph.add_node(format!("L({})", x))
            },
            Item::RuleName(x) => {
                self.add_rule_name_nodes(graph, x.clone())
            },
            _ => {
                graph.add_node("DUMMY".to_string())
            }
      }
    }

*/

    
}
