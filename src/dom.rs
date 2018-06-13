use std::collections;

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
}

#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap
}

pub type AttrMap = collections::HashMap<String, String>;

// Node constructor functions

pub fn make_text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data)
    }
}

pub fn make_element(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs
        }),
    }
}
