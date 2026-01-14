use std::{cell::RefCell, collections::{HashMap, VecDeque}, fmt::format};

use ieee1800_2023_ast::ast::{AST, Expression, Item, Sequence};
use petgraph::{Graph, graph::{self, DiGraph, NodeIndex}};

use crate::{config::Config, paths::{Node, PathFinder, PathMap}, recursion::Checker};



#[derive(Debug)]
pub struct RuleNodeMap {
    map_ : HashMap<String, (NodeIndex, NodeIndex)>,
}

impl RuleNodeMap  {
    pub fn get(&self, k : String) -> (NodeIndex, NodeIndex) {
        self.map_.get(&k).unwrap().clone()
    }
    
}

struct RuleNodeMapBuilder {
    pathmap_ : Option<PathMap>,
    graph_ : Option<Graph<String, u32>>
}

impl RuleNodeMapBuilder {
    pub fn new() -> Self {
        RuleNodeMapBuilder { pathmap_: None, graph_: None }
    }

    pub fn pathmap(mut self, pathmap : PathMap) -> Self {
        self.pathmap_ = Some(pathmap);
        self
    }

    pub fn graph(mut self, graph: Graph<String, u32>) -> Self {
         self.graph_ = Some(graph);
         self
    }

    pub fn build(mut self) -> (Graph<String, u32>, RuleNodeMap)  {

        let mut graph = self.graph_.take().expect("graph not set");
        let pathmap = self.pathmap_.take().expect("pathmap not set");


        let mut map = HashMap::new();

        for rule_name in pathmap.get_rule_names() {
            let start_node = graph.add_node(format!("{}[", rule_name));
            let end_node = graph.add_node(format!("]{}", rule_name));

            map.insert(rule_name, (start_node, end_node));
        }


        let rnmp = RuleNodeMap {
            map_ : map
        };


        (graph, rnmp)




    }
    
}







pub struct Grapher {
    graph_ : Graph<String, u32>,
    pathmap_ : PathMap,
}


impl Grapher {
    pub fn create_graph(mut self) -> Graph<String,u32> {
        let (graph, map) = RuleNodeMapBuilder::new()
            .graph(self.graph_)
            .pathmap(self.pathmap_.clone())
            .build();


        self.graph_ = graph;

        println!("rnm: {:#?}", map);

        self.update_graph(&map);

        let entry_point = self.pathmap_.get_entry_point();


        let start_node = self.graph_.add_node("START".to_string());
        let next_node = map.get(entry_point).0;
        self.graph_.add_edge(start_node, next_node, 5);





        self.graph_

    }


    fn update_graph(&mut self, map : &RuleNodeMap) {
        for (rule_name, nodes) in self.pathmap_.get_rules() {
//            self.parse_path_nodes(map, rule_name, nodes);

        }



    }


    fn parse_path_nodes(&mut self, map : &RuleNodeMap, name : String, nodes : Vec<Node>) {
        let (start_node, end_node) = map.get(name);

        let mut parent_node = start_node;

        let mut start_junctions : VecDeque<NodeIndex> = VecDeque::new();
        let mut end_junctions : VecDeque<NodeIndex> = VecDeque::new();


        for node in nodes {
            match node {
                Node::Literal(label) => {
                    let node = self.graph_.add_node(label);
                    self.graph_.add_edge(parent_node, node, 5);
                    parent_node = node;

                },
                _ => {}
            
            }
        }


        self.graph_.add_edge(parent_node, end_node, 5);

    }

    
}


impl From<PathMap> for Grapher {
    fn from(value: PathMap) -> Self {
        Grapher { graph_: DiGraph::<String, u32>::new(), pathmap_: value }
    }
}

