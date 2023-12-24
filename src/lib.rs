use std::error::Error;
use crate::bdd::BDDError;

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

pub trait Evaluate {
    type Err;
    fn assign_vars(&mut self, values: &[bool]) -> Result<(), Self::Err>;
    fn eval(&self) -> Result<bool, Self::Err>;
    fn truth_table(&self) -> String;
}
