use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;
use crate::bdd::BDDError::ParseError;
use crate::Variable;

#[cfg(test)]
mod decision_node_test;

#[cfg(test)]
mod binary_decision_diagram_test;

#[derive(Debug)]
pub enum BDDError {
    ParseError(&'static str),
}

impl From<ParseIntError> for BDDError {
    fn from(_: ParseIntError) -> Self {
        ParseError("Could not parse int")
    }
}

pub struct BinaryDecisionDiagram {
    variables: HashMap<usize, Variable>,
    nodes: HashMap<usize, BinaryNode>,
}

mod parse;

#[derive(Debug, PartialEq)]
enum BinaryNode {
    Decision(DecisionNode),
    Terminal(bool),
}

#[derive(Debug, PartialEq)]
struct DecisionNode {
    pub variable_id: usize,
    decision_map: [usize; 2],
}

impl DecisionNode {
    pub fn new_node(node_if_false: usize, node_if_true: usize, variable_id: usize) -> Self {
        Self {
            variable_id,
            decision_map: [node_if_false, node_if_true],
        }
    }

    pub fn evaluate(self, variable: &Variable) -> Result<usize, &'static str> {
        match variable.value {
            Some(false) => Ok(self.decision_map[0]),
            Some(true) => Ok(self.decision_map[1]),
            None => Err("Cannot evaluate node for an unassigned variable."),
        }
    }
}
