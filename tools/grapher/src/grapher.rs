use crate::paths::PathMap;
use crate::paths::Node;


pub struct RailRoadConverter {
    map_ : PathMap
}


impl RailRoadConverter {
    pub fn dsl(&self) -> String {
        let mut output = String::new();
        for (name, declaration) in self.map_.get_rules() {

            output = format!("{}[`{}` [{}]]\n", output, name, self.parse_node(&declaration));
        }

        output
    }

    pub fn parse_node(&self, node : &Node) -> String {

        match node {
            Node::Rule(label) => {
                format!("'{}'", label)
            },
            Node::Literal(label) => {
                format!("\"{}\"", label)
            },

            Node::RegEx(label) => {
                format!("'RE'")
            },
            Node::Alternative(node) => {
                let mut retval = format!("<");
                let mut contents = Vec::new();
                for sequence in node.iter() {
                    contents.push(self.parse_node(&sequence));
                }

                let substr = contents.join(", ");
                retval = format!("{}{}", retval, substr);

                format!("{}>", retval)
            },

            Node::Sequence(nodes) => {
                let mut retval = format!("[");
                let mut contents = Vec::new();
                for node in nodes.iter() {
                    contents.push(self.parse_node(&node));
                }

                let substr = contents.join(" ");
                retval = format!("{}{}", retval, substr);

                format!("{}]", retval)
            }


            Node::Optional(node) => {
                format!("[{}]?", self.parse_node(node))
            }

            Node::Repetition(node) => {
                format!("*[{}]", self.parse_node(node))
            }
        }
    }


}

impl From<PathMap> for RailRoadConverter {
    fn from(value: PathMap) -> Self {
        RailRoadConverter { map_: value }
    }
}





