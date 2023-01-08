#[derive(Clone)]
pub struct Property {
    pub name: String,
    pub value: String,
}

impl Property {
    pub fn new(name: &str, value: &str) -> Property {
        Property {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

impl std::fmt::Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "vp[{}->{}]", self.name, self.value)
    }
}
