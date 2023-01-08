use super::{edge::Edge, graph::Graph, vertex::Vertex};

pub struct GraphEdge {
    pub g: Graph,
    pub e: Edge,
}

impl GraphEdge {
    pub fn from(&mut self, from: Vertex) -> &mut GraphEdge {
        self.e.from = from.id;
        // Modify the edge in the graph with the same id
        for e in &mut self.g.edges._values {
            if e.id == self.e.id {
                e.from = from.id;
            }
        }
        self
    }

    pub fn to(&mut self, to: Vertex) -> Graph {
        // Modify the edge in the graph with the same id
        for e in &mut self.g.edges._values {
            if e.id == self.e.id {
                e.to = to.id;
            }
        }
        self.g.clone()
    }
}
