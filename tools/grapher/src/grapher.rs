use crate::paths::PathMap;
use crate::paths::Node as PathNode;
use railroad::{self, Node as RailroadNode, Diagram, Sequence, Choice, Terminal, NonTerminal, Empty, Stylesheet, Comment};


pub struct RailRoadConverter {
    map_: PathMap,
    expand_depth_: usize,
}


impl RailRoadConverter {
    pub fn new(map: PathMap, expand_depth: usize) -> Self {
        RailRoadConverter {
            map_: map,
            expand_depth_: expand_depth,
        }
    }

    /// Generate a diagram for the entry point rule
    pub fn generate_diagram(&self) -> Diagram<Sequence<Box<dyn RailroadNode>>> {
        let entry_point = self.map_.get_entry_point();
        let root_node = self.map_.get_rule(entry_point.clone());

        let content = self.convert_node(&root_node, 0);

        let mut seq: Sequence<Box<dyn RailroadNode>> = Sequence::default();
        seq.push(Box::new(Comment::new(entry_point)));
        seq.push(Box::new(railroad::Start));
        seq.push(content);
        seq.push(Box::new(railroad::End));

        Diagram::new_with_stylesheet(seq, &Stylesheet::Dark)
    }

    /// Convert our PathNode to a railroad Node
    fn convert_node(&self, node: &PathNode, current_depth: usize) -> Box<dyn RailroadNode> {
        match node {
            PathNode::Rule(label) => {
                if current_depth < self.expand_depth_ {
                    if let Some(rule_node) = self.map_.get_rule_opt(label) {
                        self.convert_node(&rule_node, current_depth + 1)
                    } else {
                        Box::new(NonTerminal::new(label.clone()))
                    }
                } else {
                    Box::new(NonTerminal::new(label.clone()))
                }
            }

            PathNode::Literal(label) => {
                Box::new(Terminal::new(label.clone()))
            }

            PathNode::RegEx(pattern) => {
                // Display regex patterns as terminals with a distinct format
                Box::new(Terminal::new(format!("/{}/", pattern)))
            }

            PathNode::Alternative(alternatives) => {
                if alternatives.len() == 1 {
                    // Single alternative - no need for Choice
                    self.convert_node(&alternatives[0], current_depth)
                } else {
                    let mut choice: Choice<Box<dyn RailroadNode>> = Choice::new(vec![]);
                    for alt in alternatives.iter() {
                        choice.push(self.convert_node(alt, current_depth));
                    }
                    Box::new(choice)
                }
            }

            PathNode::Sequence(nodes) => {
                if nodes.is_empty() {
                    Box::new(Empty)
                } else if nodes.len() == 1 {
                    // Single element - no need for Sequence wrapper
                    self.convert_node(&nodes[0], current_depth)
                } else {
                    let mut seq: Sequence<Box<dyn RailroadNode>> = Sequence::new(vec![]);
                    for n in nodes.iter() {
                        seq.push(self.convert_node(n, current_depth));
                    }
                    Box::new(seq)
                }
            }

            PathNode::Optional(inner) => {
                let inner_node = self.convert_node(inner, current_depth);
                Box::new(railroad::Optional::new(inner_node))
            }

            PathNode::Repetition(inner) => {
                let inner_node = self.convert_node(inner, current_depth);
                // Repeat takes (forward_element, backward_element)
                // For zero-or-more, we use Empty as the backward path
                Box::new(railroad::Repeat::new(inner_node, Box::new(Empty) as Box<dyn RailroadNode>))
            }
        }
    }
}
