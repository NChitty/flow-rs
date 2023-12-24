use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;
use crate::bdd::BDDError::{EvaluationError, ParseError};
use crate::bdd::BinaryNode::{Decision, Terminal};
use crate::Variable;

#[cfg(test)]
mod decision_node_test;

#[derive(Debug, PartialEq)]
pub enum BDDError {
    EvaluationError(&'static str),
    ParseError(&'static str),
    VariableAssignmentError(&'static str),
}

impl From<ParseIntError> for BDDError {
    fn from(_: ParseIntError) -> Self {
        ParseError("Could not parse int")
    }
}

pub struct BinaryDecisionDiagram {
    variables: HashMap<usize, Variable>,
    nodes: HashMap<usize, BinaryNode>,
    entry_node: usize,
}

mod parse;
mod eval;

#[derive(Debug, PartialEq)]
enum BinaryNode {
    Decision(DecisionNode),
    Terminal(bool),
}

impl BinaryNode {
    fn is_decision_node(&self) -> bool {
        match self {
            Decision(_) => true,
            Terminal(_) => false,
        }
    }

    fn get_node(&self) -> Result<&DecisionNode, BDDError> {
        match self {
            Decision(node) => Ok(node),
            Terminal(_) => Err(EvaluationError("Cannot get decision from terminal node")),
        }
    }
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

    pub fn evaluate(&self, variable: &Variable) -> Result<usize, BDDError> {
        match variable.value {
            Some(false) => Ok(self.decision_map[0]),
            Some(true) => Ok(self.decision_map[1]),
            None => Err(EvaluationError("Cannot evaluate node for an unassigned variable.")),
        }
    }
}
