use super::edge::Edge;

#[derive(Clone)]
pub struct Edges {
    pub _values: Vec<Edge>,
}

impl Edges {
    pub fn new() -> Edges {
        Edges {
            _values: Vec::new(),
        }
    }

    pub fn id(&self, id: i32) -> Option<Edge> {
        for e in &self._values {
            if e.id == id {
                return Some(e.to_owned());
            }
        }
        None
    }
}

impl std::fmt::Display for Edges {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Display each edge on a new line and don't display the last new line
        let mut s = String::new();
        for e in &self._values {
            s.push_str(&format!("{}\n", e));
        }
        write!(f, "{}", s.trim_end())
    }
}
