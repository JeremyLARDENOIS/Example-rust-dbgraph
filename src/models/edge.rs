use crate::get_id;

use super::graph::Graph;

#[derive(Clone)]
pub struct Edge {
    pub id: i32,
    pub from: i32,
    pub to: i32,
    pub label: String,
}

impl Edge {
    pub fn new(label: &str) -> Edge {
        Edge {
            id: get_id(),
            from: -1,
            to: -1,
            label: label.to_string(),
        }
    }

    pub fn drop(&mut self, g: &mut Graph) {
        g.edges._values.retain(|e| e.id != self.id);
    }
}

impl std::fmt::Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "e[{}][{}-{}>{}]",
            self.id, self.from, self.label, self.to
        )
    }
}
