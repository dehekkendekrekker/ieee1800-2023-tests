use petgraph::graph::Graph;
use petgraph::visit::EdgeRef;
use petgraph::Directed;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use std::io::Cursor;

pub fn to_graphml<N, E>(graph: &Graph<N, E, Directed>) -> String
where
    N: std::fmt::Display,
    E: std::fmt::Display,
{
    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);
    // XML declaration
    writer
        .write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))
        .unwrap();
    // GraphML root
    let mut graphml = BytesStart::new("graphml");
    graphml.push_attribute(("xmlns", "http://graphml.graphdrawing.org/xmlns"));
    graphml.push_attribute(("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance"));
    graphml.push_attribute(("xsi:schemaLocation", "http://graphml.graphdrawing.org/xmlns http://graphml.graphdrawing.org/xmlns/1.0/graphml.xsd",));
    writer.write_event(Event::Start(graphml)).unwrap();


    // Add key definitions for Gephi
let mut key_label = BytesStart::new("key");
key_label.push_attribute(("id", "label"));
key_label.push_attribute(("for", "node"));
key_label.push_attribute(("attr.name", "label"));
key_label.push_attribute(("attr.type", "string"));
writer.write_event(Event::Empty(key_label)).unwrap();

let mut key_weight = BytesStart::new("key");
key_weight.push_attribute(("id", "weight"));
key_weight.push_attribute(("for", "edge"));
key_weight.push_attribute(("attr.name", "weight"));
key_weight.push_attribute(("attr.type", "string"));
writer.write_event(Event::Empty(key_weight)).unwrap();


    // Graph element
    let mut graph_elem = BytesStart::new("graph");
    graph_elem.push_attribute(("id", "G"));
    graph_elem.push_attribute(("edgedefault", "directed"));
    writer.write_event(Event::Start(graph_elem)).unwrap();
    // Write nodes
    for node in graph.node_indices() {
        let mut node_elem = BytesStart::new("node");
        node_elem.push_attribute(("id", format!("n{}", node.index()).as_str()));
        writer.write_event(Event::Start(node_elem.clone())).unwrap();
        // Write node data
        let mut data = BytesStart::new("data");
        data.push_attribute(("key", "label"));
        writer.write_event(Event::Start(data.clone())).unwrap();
        writer
            .write_event(Event::Text(BytesText::new(&format!(
                "{}",
                graph[node]
            ))))
            .unwrap();
        writer.write_event(Event::End(BytesEnd::new("data"))).unwrap();
        writer.write_event(Event::End(BytesEnd::new("node"))).unwrap();
    }
    // Write edges
    for edge in graph.edge_references() {
        let mut edge_elem = BytesStart::new("edge");
        edge_elem.push_attribute(("source", format!("n{}", edge.source().index()).as_str()));
        edge_elem.push_attribute(("target", format!("n{}", edge.target().index()).as_str()));
        writer.write_event(Event::Start(edge_elem.clone())).unwrap();
        // Write edge data
        let mut data = BytesStart::new("data");
        data.push_attribute(("key", "weight"));
        writer.write_event(Event::Start(data.clone())).unwrap();
        writer
            .write_event(Event::Text(BytesText::new(&format!(
                "{}",
                edge.weight()
            ))))
            .unwrap();
        writer.write_event(Event::End(BytesEnd::new("data"))).unwrap();
        writer.write_event(Event::End(BytesEnd::new("edge"))).unwrap();
    }
    // Close graph
    writer.write_event(Event::End(BytesEnd::new("graph"))).unwrap();
    // Close graphml
    writer.write_event(Event::End(BytesEnd::new("graphml"))).unwrap();
    String::from_utf8(writer.into_inner().into_inner()).unwrap()
}
