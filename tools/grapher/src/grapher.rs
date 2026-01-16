use crate::paths::PathMap;
use crate::paths::Node;


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

    pub fn dsl(&self) -> String {
        let mut output = String::new();
        for (name, declaration) in self.map_.get_rules() {
            output = format!("{}[`{}` [{}]]\n", output, name, self.parse_node(&declaration, 0));
        }

        output
    }

    fn parse_node(&self, node: &Node, current_depth: usize) -> String {
        match node {
            Node::Rule(label) => {
                if current_depth < self.expand_depth_ {
                    if let Some(rule_node) = self.map_.get_rule_opt(label) {
                        self.parse_node(&rule_node, current_depth + 1)
                    } else {
                        format!("'{}'", label)
                    }
                } else {
                    format!("'{}'", label)
                }
            }
            Node::Literal(label) => {
                format!("\"{}\"", label)
            }

            Node::RegEx(_) => {
                format!("'RE'")
            }
            Node::Alternative(node) => {
                let mut retval = format!("<");
                let mut contents = Vec::new();
                for sequence in node.iter() {
                    contents.push(self.parse_node(&sequence, current_depth));
                }

                let substr = contents.join(", ");
                retval = format!("{}{}", retval, substr);

                format!("{}>", retval)
            }

            Node::Sequence(nodes) => {
                let mut retval = format!("[");
                let mut contents = Vec::new();
                for node in nodes.iter() {
                    contents.push(self.parse_node(&node, current_depth));
                }

                let substr = contents.join(" ");
                retval = format!("{}{}", retval, substr);

                format!("{}]", retval)
            }

            Node::Optional(node) => {
                format!("[{}]?", self.parse_node(node, current_depth))
            }

            Node::Repetition(node) => {
                format!("*[{}]", self.parse_node(node, current_depth))
            }
        }
    }
}





