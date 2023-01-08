use super::{
    edge::Edge, edges::Edges, graphedge::GraphEdge, graphvertex::GraphVertex, vertex::Vertex,
    vertices::Vertices,
};

#[derive(Clone)]
pub struct Graph {
    pub vertices: Vertices,
    pub edges: Edges,
}

#[allow(non_snake_case)]
impl Graph {
    pub fn new() -> Graph {
        Graph {
            vertices: Vertices::new(),
            edges: Edges::new(),
        }
    }

    pub fn drop(&mut self) -> Graph {
        Graph::new()
    }

    pub fn V(&mut self) -> &Vertices {
        &self.vertices
    }

    pub fn E(&self) -> &Edges {
        &self.edges
    }

    pub fn addV(&mut self) -> GraphVertex {
        let v = Vertex::new();
        self.vertices._values.push(v);
        let v = self.vertices._values.last().unwrap();
        GraphVertex {
            g: self.clone(),
            v: v.clone(),
        }
    }

    pub fn addE(&mut self, label: &str) -> GraphEdge {
        let e = Edge::new(label);
        self.edges._values.push(e.clone());
        GraphEdge { g: self.clone(), e }
    }
}

impl std::fmt::Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Graph {{ vertices: {}, edges: {} }}",
            self.vertices._values.len(),
            self.edges._values.len()
        )
    }
}
