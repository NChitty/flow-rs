/*
 * Copyright (c) 2023 William Nicholas Chitty
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

use crate::FlowError::{EvaluationError, ParseError, VariableAssignmentError};

pub mod bdd;

pub type Variable = Option<bool>;

#[derive(Debug, PartialEq)]
pub enum FlowError {
    EvaluationError(&'static str),
    ParseError(&'static str),
    VariableAssignmentError(&'static str),
}

impl From<ParseIntError> for FlowError {
    fn from(_: ParseIntError) -> Self { ParseError("Could not parse int") }
}

impl Display for FlowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EvaluationError(msg) => {
                write!(f, "Could not evaluate: {msg}")
            },
            ParseError(msg) => {
                write!(f, "Could not parse: {msg}")
            },
            VariableAssignmentError(msg) => {
                write!(f, "Could not assign variable: {msg}")
            },
        }
    }
}

impl Error for FlowError {}

pub trait Evaluate {
    /// Evaluate the current struct using currently assigned variables.
    /// # Errors
    /// * `VariableAssignmentError` - tried to valuate on a variable that has
    ///   not been assigned
    /// * `EvaluationError`
    /// # Example
    /// The following assigns the variable `0` to true for the BDD and gets the
    /// result via `eval`.
    /// ```
    /// use flow::bdd::BinaryDecisionDiagram;
    /// use flow::Evaluate;
    /// const SIMPLE_BDD: &str = "vars 1
    ///  nodes 3
    ///  0 1 2 0
    ///  1 -1 -1 1
    ///  2 -1 -1 0";
    /// let mut some_evaluate: BinaryDecisionDiagram = SIMPLE_BDD.parse().unwrap();
    /// let eval: bool = some_evaluate.eval(&vec![true]).unwrap();
    /// ```
    fn eval(&self, values: &[bool]) -> Result<bool, FlowError>;

    /// Get a list of booleans in order based on values of the variables
    /// # Errors
    /// * `VariableAssignmentError` - when the number does not match or a match
    ///   can't be found
    /// * `EvaluationError`
    /// # Example
    /// The resulting vector of booleans is indexed 0 for variable 0 false and 1
    /// for variable 0 true.
    /// ```
    /// use flow::bdd::BinaryDecisionDiagram;
    /// use flow::Evaluate;
    /// const SIMPLE_BDD: &str = "vars 1
    ///  nodes 3
    ///  0 1 2 0
    ///  1 -1 -1 1
    ///  2 -1 -1 0";
    /// let some_evaluate: BinaryDecisionDiagram = SIMPLE_BDD.parse().unwrap();
    /// some_evaluate.truth_table();
    /// ```
    fn truth_table(&self) -> Result<Vec<bool>, FlowError>;
}

pub(crate) fn convert_bits_to_bools(bits: usize, num_vars: usize) -> Vec<bool> {
    let mut bools = Vec::new();
    let mut cur_bits = bits;
    let mut tracker = num_vars;
    while tracker > 0 {
        bools.push((cur_bits & 1) == 1);
        tracker -= 1;
        cur_bits >>= 1;
    }
    bools
}

#[must_use]
pub fn byte_to_bools(byte: u8) -> Vec<bool> {
    let mut bools = Vec::new();
    let mut cur_bits = byte;
    let mut tracker = 8;
    while tracker > 0 {
        bools.push((cur_bits & 1) == 1);
        tracker -= 1;
        cur_bits >>= 1;
    }
    bools
}

#[cfg(test)]
mod test {
    use crate::byte_to_bools;

    #[test]
    fn byte_to_bool() {
        let byte = 0xaa;
        let expected = vec![false, true, false, true, false, true, false, true];
        let actual = byte_to_bools(byte);
        assert_eq!(actual, expected);
    }
}
