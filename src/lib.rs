pub mod bdd;

pub struct Variable {
    pub value: Option<bool>,
}

impl Variable {
    pub fn new() -> Self {
        Self {
            value: None,
        }
    }
}
