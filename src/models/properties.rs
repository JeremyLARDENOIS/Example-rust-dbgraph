use super::property::Property;

#[derive(Clone)]
pub struct Properties {
    pub _values: Vec<Property>,
}

impl Properties {
    pub fn new() -> Properties {
        Properties {
            _values: Vec::new(),
        }
    }
}
