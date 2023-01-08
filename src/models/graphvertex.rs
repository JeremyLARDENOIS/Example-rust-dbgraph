use super::{graph::Graph, property::Property, vertex::Vertex};

pub struct GraphVertex {
    pub g: Graph,
    pub v: Vertex,
}

impl GraphVertex {
    pub fn property(&mut self, name: &str, value: &str) -> Graph {
        // Modify the vertex in the graph with the same id
        for v in &mut self.g.vertices._values {
            if v.id == self.v.id {
                v.properties._values.push(Property::new(name, value));
            }
        }
        self.g.clone()
    }
}
