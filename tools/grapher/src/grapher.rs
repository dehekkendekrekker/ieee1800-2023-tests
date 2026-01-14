use std::{cell::RefCell, collections::HashMap};

use ieee1800_2023_ast::ast::{AST, Expression, Item, Sequence};
use petgraph::{Graph, graph::{self, DiGraph, NodeIndex}};

use crate::{config::Config, paths::{Node, PathFinder}, recursion::Checker};







pub struct Grapher {
    graph_ : Graph<String, u32>,
    path_finder_  : PathFinder
}


impl Grapher {
    pub fn new(path_finder : PathFinder) -> Self {
        Grapher {  
            graph_ : DiGraph::<String, u32>::new(),
            path_finder_ : path_finder

        }
    }
}
/*
    pub fn create_graph(&mut self) -> Graph<String, u32> {
//        let entry_point = self.config_.entry_point.clone();

    }
}
*/


