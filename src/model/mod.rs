use crate::config;
use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::visit::IntoNodeReferences;
use petgraph::{self, Graph};
use slog::Logger;
use std::collections::HashMap;

type GraphType = Graph<Node, Metrics, petgraph::Directed>;
#[derive(Debug)]
pub struct ObsGraph {
    name: String,
    display_name: Option<String>,
    entry_point: String,
    graph: GraphType,
    root: bool,
}

impl ObsGraph {
    pub fn new<S: Into<String>>(name: S, entry_point: S) -> ObsGraph {
        ObsGraph {
            name: name.into(),
            display_name: None,
            entry_point: entry_point.into(),
            graph: Graph::new(),
            root: false,
        }
    }

    pub fn root<S: Into<String>>(name: S, entry_point: S) -> ObsGraph {
        ObsGraph {
            name: name.into(),
            display_name: None,
            entry_point: entry_point.into(),
            graph: Graph::new(),
            root: true,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_root(&self) -> bool {
        self.root
    }

    pub fn display_name(&self) -> &str {
        self.display_name.as_ref().unwrap_or(&self.name).as_str()
    }

    pub fn entry_point(&self) -> &str {
        &self.entry_point
    }

    pub fn set_display_name<S: Into<String>>(&mut self, display_name: S) {
        self.display_name = Some(display_name.into());
    }

    pub fn add_node(&mut self, n: Node) -> NodeIndex<u32> {
        self.graph.add_node(n)
    }

    pub fn find_node_by_name(&self, name: &str) -> Option<(NodeIndex<u32>, &Node)> {
        self.graph
            .node_references()
            .find(|(_idx, n)| n.name() == name)
    }

    pub fn add_edge_by_name(
        &mut self,
        src: &str,
        tgt: &str,
        metrics: Metrics,
    ) -> Option<EdgeIndex<u32>> {
        let src_idx = match self.find_node_by_name(src) {
            Some((idx, _n)) => idx,
            None => return None,
        };
        let tgt_idx = match self.find_node_by_name(tgt) {
            Some((idx, n)) => idx,
            None => return None,
        };
        Some(self.add_edge(src_idx, tgt_idx, metrics))
    }

    pub fn add_edge(
        &mut self,
        src: NodeIndex<u32>,
        tgt: NodeIndex<u32>,
        metrics: Metrics,
    ) -> EdgeIndex<u32> {
        self.graph.add_edge(src, tgt, metrics)
    }

    pub fn graph(&self) -> &GraphType {
        &self.graph
    }
}

#[derive(Debug)]
pub struct World {
    logger: Logger,
    graphs: HashMap<String, Node>,
}

impl World {
    pub fn new(logger: Logger) -> World {
        World {
            logger,
            graphs: HashMap::default(),
        }
    }

    pub fn extend_from_config(&mut self, config: &config::FileConfig) {
        config.regions.iter().for_each(|n| {
            if self.graphs.contains_key(&n.name) {
                warn!(
                    self.logger,
                    "Graph already contains root {}, skipping", n.name
                );
            } else {
                self.graphs
                    .insert(n.name.to_string(), from_config_node(&self.logger, &n, true));
            }
        });
    }

    pub fn graph(&self, name: &str) -> Option<&Node> {
        self.graphs.get(name)
    }

    pub fn graph_names(&self) -> Vec<String> {
        let mut keys = self
            .graphs
            .keys()
            .map(|k| k.to_string())
            .collect::<Vec<String>>();
        keys.sort();
        keys
    }
}

fn graph_name(name: &str, parent: Option<&str>) -> String {
    parent
        .map(|p| p.to_owned() + "/" + name)
        .unwrap_or_else(|| name.to_string())
}

fn from_config_node(logger: &Logger, node: &config::Node, root: bool) -> Node {
    if let Some(children) = node.nodes.as_ref().filter(|v| !v.is_empty()) {
        debug!(logger, "Creating new graph node '{}'", node.name);
        let mut g = if root {
            ObsGraph::root(
                node.name.to_owned(),
                node.entry_point
                    .as_ref()
                    .map(|e| e.to_string())
                    .unwrap_or_else(|| "INTERNET".to_string()),
            )
        } else {
            ObsGraph::new(
                node.name.to_owned(),
                node.entry_point
                    .as_ref()
                    .map(|e| e.to_string())
                    .unwrap_or_else(|| "INTERNET".to_string()),
            )
        };
        if let Some(d) = node.display_name.as_ref() {
            g.set_display_name(d.as_str());
        }
        children
            .iter()
            .map(move |c| from_config_node(logger, c, false))
            .for_each(|n| {
                trace!(logger, "Adding node '{}' to graph", n.name());
                g.add_node(n);
            });
        if let Some(connections) = node.connections.as_ref() {
            connections.iter().for_each(|c| {
                trace!(logger, "Adding connections");
                c.targets.iter().for_each(|tgt| {
                    if g.add_edge_by_name(&c.source, &tgt, Metrics::empty())
                        .is_some()
                    {
                        trace!(logger, "Connected {} => {}", c.source, &tgt);
                    } else {
                        warn!(logger, "Couldn't add connection {} => {}", c.source, tgt);
                    }
                });
            });
        }
        Node::Graph(g)
    } else {
        debug!(logger, "Creating leaf node '{}'", node.name);
        Node::Leaf {
            name: node.name.to_owned(),
            display_name: node.display_name.clone(),
        }
    }
}

#[derive(Debug)]
pub enum Node {
    Leaf {
        name: String,
        display_name: Option<String>,
    },
    Graph(ObsGraph),
}

impl Node {
    pub fn name(&self) -> &str {
        match self {
            Node::Leaf {
                name,
                display_name: _,
            } => &name,
            Node::Graph(g) => g.name(),
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Node::Leaf { name, display_name } => display_name.as_ref().unwrap_or(name).as_str(),
            Node::Graph(g) => g.display_name(),
        }
    }
}

#[derive(Debug)]
pub struct Metrics {
    pub normal: f64,
    pub warning: f64,
    pub danger: f64,
}

impl Metrics {
    pub fn empty() -> Metrics {
        Metrics {
            normal: 100.0,
            warning: 10.0,
            danger: 1.0,
        }
    }

    pub fn new(normal: f64, warning: f64, danger: f64) -> Metrics {
        Metrics {
            normal,
            warning,
            danger,
        }
    }
}

#[cfg(test)]
mod test {
    use super::World;
    use crate::config::FileConfig;
    use crate::logging;
    use serde_yaml;
    use slog::Level;
    use std::fs::File;

    #[test]
    fn load_parsed_file() {
        let f = File::open("sample/simple.yml").unwrap();
        let c: FileConfig = serde_yaml::from_reader(&f).unwrap();
        let logger = logging::root_logger();
        logging::set_global_level(Level::Trace);
        let mut world = World::new(logger);
        world.extend_from_config(&c);
        dbg!(world);
    }
}
