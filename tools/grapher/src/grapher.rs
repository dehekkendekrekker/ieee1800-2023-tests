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
        let node = graph.add_node(rule_name.clone());
        graph.add_edge(parent_node, node, 5);
        let expression = self.ast_.get_rule(&rule_name).expect("Rule not found");
        self.add_expression_nodes(graph, node, expression)
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
            let mut to_nodes = vec![];
            for from_node in &from_nodes {
                to_nodes.extend(self.add_item_nodes(graph, from_node.clone(), item));
            }
            from_nodes = to_nodes;
        }
        
        from_nodes
    }

    pub fn add_item_nodes(&self, graph: &mut Graph<String, u8>, from_node: NodeIndex, item : &Item) -> Vec<NodeIndex> {
        // Adds item nodes. The parent will be connected to the child node. The child node should
        // be returned.


        match item {
            Item::Literal(literal) => {
                let node = graph.add_node(format!("L({})", literal));
                graph.add_edge(from_node, node, 5);
                vec![node]
            },
            Item::RegEx(regex) => {
                let node = graph.add_node(format!("RE({})", regex));
                graph.add_edge(from_node, node, 5);
                vec![node]
            },
            Item::Optional(_) => {
                let node = graph.add_node(format!("OPTIONAL"));
                graph.add_edge(from_node, node, 5);
                vec![node]
            },
            Item::Repetition(_) => {
                let node = graph.add_node(format!("REPETITION"));
                graph.add_edge(from_node, node, 5);
                vec![node]
            }
            Item::RuleName(rule_name) => {
                self.add_rule_name_nodes(graph, from_node, rule_name.clone())
            }
        }
    }
}


