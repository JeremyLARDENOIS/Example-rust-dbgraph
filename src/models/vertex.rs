use crate::get_id;

use super::{edges::Edges, graph::Graph, properties::Properties, vertices::Vertices};

#[derive(Clone)]
pub struct Vertex {
    pub id: i32,
    pub label: String,
    pub properties: Properties,
}

#[allow(non_snake_case)]
impl Vertex {
    pub fn new() -> Vertex {
        Vertex {
            id: get_id(),
            label: String::new(),
            properties: Properties::new(),
        }
    }

    pub fn drop(&mut self, g: &mut Graph) {
        g.vertices._values.retain(|v| v.id != self.id);
        g.edges
            ._values
            .retain(|e| e.from != self.id && e.to != self.id);
    }

    pub fn inE(&self, g: Graph) -> Edges {
        let mut edges = Edges::new();
        for e in g.edges._values {
            if e.to == self.id {
                edges._values.push(e.to_owned());
            }
        }
        edges
    }

    pub fn outE(&self, g: Graph) -> Edges {
        let mut edges = Edges::new();
        for e in g.edges._values {
            if e.from == self.id {
                edges._values.push(e.to_owned());
            }
        }
        edges
    }

    pub fn inV(&self, g: Graph) -> Vertices {
        let mut vertices = Vertices::new();
        for e in g.edges._values {
            if e.to == self.id {
                vertices._values.push(g.vertices.id(e.from).unwrap());
            }
        }
        vertices
    }

    pub fn outV(&self, g: Graph) -> Vertices {
        let mut vertices = Vertices::new();
        for e in g.edges._values {
            if e.from == self.id {
                vertices._values.push(g.vertices.id(e.to).unwrap());
            }
        }
        vertices
    }
}

impl std::fmt::Display for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "v[{}]", self.id)
    }
}
