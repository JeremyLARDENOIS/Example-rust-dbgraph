use super::vertex::Vertex;

#[derive(Clone)]
pub struct Vertices {
    pub _values: Vec<Vertex>,
}

impl Vertices {
    pub fn new() -> Vertices {
        Vertices {
            _values: Vec::new(),
        }
    }

    pub fn has(&self, property_name: &str, property_value: &str) -> Vertices {
        let mut vertices = Vertices::new();
        for v in &self._values {
            for p in &v.properties._values {
                if p.name == property_name && p.value == property_value {
                    vertices._values.push(v.to_owned());
                }
            }
        }
        vertices
    }
    pub fn id(&self, id: i32) -> Option<Vertex> {
        for v in &self._values {
            if v.id == id {
                return Some(v.to_owned());
            }
        }
        None
    }
}

impl std::fmt::Display for Vertices {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Display each vertex on a new line and don't display the last new line
        let mut s = String::new();
        for v in &self._values {
            s.push_str(&format!("{}\n", v));
        }
        write!(f, "{}", s.trim_end())
    }
}
