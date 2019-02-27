use serde_derive::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

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
