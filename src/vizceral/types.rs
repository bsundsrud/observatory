use crate::model::{Metrics, Node};
use serde_derive::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::convert::From;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum VizceralRenderer {
    #[serde(rename = "global")]
    Global,
    #[serde(rename = "region")]
    Region,
    #[serde(rename = "focusedChild")]
    FocusedChild,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VizceralNode {
    renderer: VizceralRenderer,
    name: String,
    display_name: Option<String>,
    entry_node: Option<String>,
    updated: Option<i64>,
    max_volume: Option<i64>,
    class: Option<String>,
    nodes: Option<Vec<VizceralNode>>,
    connections: Option<Vec<VizceralConnection>>,
    notices: Option<Vec<VizceralNotice>>,
}
#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum VizceralSeverity {
    Info = 0,
    Warning = 1,
    Error = 2,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VizceralNotice {
    title: String,
    link: Option<String>,
    severity: Option<VizceralSeverity>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VizceralConnection {
    source: String,
    target: String,
    metrics: Option<VizceralMetric>,
    notices: Option<Vec<VizceralNotice>>,
    class: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VizceralMetric {
    normal: Option<f64>,
    danger: Option<f64>,
    warning: Option<f64>,
}

impl From<&Node> for VizceralNode {
    fn from(n: &Node) -> VizceralNode {
        match n {
            Node::Leaf {
                name,
                display_name: _,
            } => VizceralNode {
                renderer: VizceralRenderer::FocusedChild,
                name: name.to_string(),
                display_name: Some(n.display_name().to_string()),
                entry_node: None,
                updated: None,
                max_volume: None,
                class: None,
                notices: None,
                nodes: None,
                connections: None,
            },
            Node::Graph(g) => {
                use petgraph::visit::{EdgeRef, IntoNodeReferences};
                let nodes = g
                    .graph()
                    .node_references()
                    .map(|(_idx, n)| VizceralNode::from(n))
                    .collect();
                let connections = g
                    .graph()
                    .edge_references()
                    .map(|edge_ref| {
                        let src = &g.graph()[edge_ref.source()];
                        let tgt = &g.graph()[edge_ref.target()];
                        VizceralConnection {
                            source: src.name().to_string(),
                            target: tgt.name().to_string(),
                            metrics: Some(edge_ref.weight().into()),
                            notices: None,
                            class: None,
                        }
                    })
                    .collect();
                let renderer = if g.is_root() {
                    VizceralRenderer::Global
                } else {
                    VizceralRenderer::Region
                };

                VizceralNode {
                    renderer,
                    name: g.name().to_string(),
                    display_name: Some(g.display_name().to_string()),
                    entry_node: Some(g.entry_point().to_string()),
                    updated: None,
                    max_volume: None,
                    class: None,
                    notices: None,
                    nodes: Some(nodes),
                    connections: Some(connections),
                }
            }
        }
    }
}

impl From<&Metrics> for VizceralMetric {
    fn from(m: &Metrics) -> VizceralMetric {
        VizceralMetric {
            normal: Some(m.normal),
            warning: Some(m.warning),
            danger: Some(m.danger),
        }
    }
}
