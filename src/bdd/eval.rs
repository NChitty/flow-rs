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

use crate::bdd::BinaryDecisionDiagram;
use crate::bdd::BinaryNode::{Decision, Terminal};
use crate::FlowError::{EvaluationError, VariableAssignmentError};
use crate::{convert_bits_to_bools, Evaluate, FlowError};

impl Evaluate for BinaryDecisionDiagram {
    fn assign_vars(&mut self, values: &[bool]) -> Result<Vec<bool>, FlowError> {
        if values.len() < self.variables.len() {
            return Err(VariableAssignmentError(
                "The length of values is less than the number of variables to assign.",
            ));
        }
        let bools = &values[0..self.variables.len()];

        let mut keys: Vec<usize> = self.variables.keys().copied().collect();
        keys.sort_unstable();

        for (index, value) in keys.into_iter().enumerate() {
            *self
                .variables
                .get_mut(&value)
                .expect("Malformed variables, unable to find in map") = Some(bools[index]);
        }

        Ok(self
            .variables
            .values()
            .map(|val| match val {
                Some(thing) => *thing,
                None => panic!("None in variable map after assignment."),
            })
            .collect())
    }

    fn eval(&self) -> Result<bool, FlowError> {
        let mut cur_node = self
            .nodes
            .get(&self.entry_node)
            .ok_or(EvaluationError("Unable to grab entry node"))?;

        loop {
            match cur_node {
                Decision(decision_node) => {
                    let var = self
                        .variables
                        .get(&decision_node.variable_id)
                        .expect("Decision variable not present in map.");
                    let next_node = decision_node.evaluate(*var)?;
                    cur_node = self
                        .nodes
                        .get(&next_node)
                        .ok_or(EvaluationError("Could not traverse to next node"))?;
                },
                Terminal(b) => return Ok(*b),
            }
        }
    }

    fn truth_table(&mut self) -> Result<Vec<bool>, FlowError> {
        if self.variables.len() > usize::BITS as usize {
            return Err(EvaluationError("Too many variables"));
        }
        let combinations: usize = 1 << self.variables.len();
        let mut results: Vec<bool> = Vec::new();

        for var_set in 0..combinations {
            let vars = convert_bits_to_bools(var_set, self.variables.len());
            self.assign_vars(&vars)?;
            results.push(self.eval()?);
        }

        Ok(results)
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::bdd::BinaryDecisionDiagram;
    use crate::Evaluate;

    const SIMPLE_BDD: &str = "vars 1
nodes 3
0 2 1 0
1 -1 -1 0
2 -1 -1 1";

    #[test]
    fn too_few_vars() {
        let mut bdd = BinaryDecisionDiagram::from_str(SIMPLE_BDD).unwrap();
        let bools = vec![];
        assert!(bdd.assign_vars(&bools).is_err());
    }

    #[test]
    fn too_many_vars() {
        let mut bdd = BinaryDecisionDiagram::from_str(SIMPLE_BDD).unwrap();
        let bools = vec![true, true];
        assert!(bdd.assign_vars(&bools).is_ok());
        assert!(bdd.variables.get(&0).unwrap().unwrap_or(false));
    }

    #[test]
    fn one_indexed_vars() {
        let mut bdd = BinaryDecisionDiagram::from_str(
            "vars 1
nodes 3
0 2 1 1
1 -1 -1 0
2 -1 -1 1",
        )
        .unwrap();
        let bools = vec![true];
        assert!(bdd.assign_vars(&bools).is_ok());
    }

    #[test]
    fn vars() {
        let mut bdd = BinaryDecisionDiagram::from_str(SIMPLE_BDD).unwrap();
        let bools = vec![true];
        assert!(bdd.assign_vars(&bools).is_ok());
        assert!(bdd.variables.get(&0).unwrap().unwrap_or(false));
    }

    #[test]
    fn true_assignment() {
        let mut bdd = BinaryDecisionDiagram::from_str(SIMPLE_BDD).unwrap();
        let bools = vec![true];
        bdd.assign_vars(&bools).expect("Could not assign bools.");
        assert!(bdd.eval().expect("Could not evaluate"))
    }

    #[test]
    fn false_assignment() {
        let mut bdd = BinaryDecisionDiagram::from_str(SIMPLE_BDD).unwrap();
        let bools = vec![false];
        bdd.assign_vars(&bools).expect("Could not assign bools.");
        assert!(!bdd.eval().expect("Could not evaluate"))
    }

    #[test]
    fn truth_table() {
        let mut bdd = BinaryDecisionDiagram::from_str(SIMPLE_BDD).unwrap();
        assert_eq!(
            vec![false, true],
            bdd.truth_table().expect("Could not complete truth table")
        );
    }
}
