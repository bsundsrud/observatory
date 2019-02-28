use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileConfig {
    regions: Vec<Node>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    name: String,
    display_name: Option<String>,
    entry_point: Option<String>,
    nodes: Vec<Node>,
    connections: Vec<Connections>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Connection {
    source: String,
    target: Vec<String>,
}
