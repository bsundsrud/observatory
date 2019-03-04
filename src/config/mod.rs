use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileConfig {
    pub regions: Vec<Node>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub name: String,
    pub display_name: Option<String>,
    pub entry_point: Option<String>,
    pub nodes: Option<Vec<Node>>,
    pub connections: Option<Vec<Connection>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Connection {
    pub source: String,
    pub targets: Vec<String>,
}

#[cfg(test)]
mod test {
    #[test]
    fn load_simple_sample() {
        use super::FileConfig;
        use serde_yaml;
        use std::fs::File;
        let f = File::open("sample/simple.yml").unwrap();
        let c: FileConfig = serde_yaml::from_reader(&f).unwrap();
        assert_eq!(1, c.regions.len());
    }
}
