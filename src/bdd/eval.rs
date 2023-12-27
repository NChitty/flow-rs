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

use crate::bdd::BDDError::{EvaluationError, VariableAssignmentError};
use crate::bdd::BinaryNode::{Decision, Terminal};
use crate::bdd::{BDDError, BinaryDecisionDiagram};
use crate::{convert_bits_to_bools, Evaluate};

impl Evaluate for BinaryDecisionDiagram {
    type Err = BDDError;

    fn assign_vars(&mut self, values: &[bool]) -> Result<(), Self::Err> {
        if values.len() != self.variables.len() {
            return Err(VariableAssignmentError(
                "Length of variable assignment does not match",
            ));
        }

        let mut keys: Vec<usize> = self.variables.keys().copied().collect();
        keys.sort_unstable();

        for (index, value) in keys.into_iter().enumerate() {
            self.variables
                .get_mut(&value)
                .expect("Malformed variables, unable to find in map") = &mut Some(values[index]);
        }

        Ok(())
    }

    fn eval(&self) -> Result<bool, Self::Err> {
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
                    let next_node = decision_node.evaluate(var)?;
                    cur_node = self
                        .nodes
                        .get(&next_node)
                        .ok_or(EvaluationError("Could not traverse to next node"))?;
                },
                Terminal(b) => return Ok(*b),
            }
        }
    }

    fn truth_table(&mut self) -> Result<Vec<bool>, Self::Err> {
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
    fn given_too_few_vars_then_err() {
        let mut bdd = BinaryDecisionDiagram::from_str(SIMPLE_BDD).unwrap();
        let bools = vec![];
        assert!(bdd.assign_vars(&bools).is_err());
    }

    #[test]
    fn given_too_many_vars_then_err() {
        let mut bdd = BinaryDecisionDiagram::from_str(SIMPLE_BDD).unwrap();
        let bools = vec![true, true];
        assert!(bdd.assign_vars(&bools).is_err());
    }

    #[test]
    fn given_one_indexed_vars_then_ok() {
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
    fn given_vars_then_assigned() {
        let mut bdd = BinaryDecisionDiagram::from_str(SIMPLE_BDD).unwrap();
        let bools = vec![true];
        assert!(bdd.assign_vars(&bools).is_ok());
        assert!(bdd.variables.get(&0).unwrap().unwrap_or(false));
    }

    #[test]
    fn given_true_assignment_when_eval_then_true() {
        let mut bdd = BinaryDecisionDiagram::from_str(SIMPLE_BDD).unwrap();
        let bools = vec![true];
        bdd.assign_vars(&bools).expect("Could not assign bools.");
        assert!(bdd.eval().expect("Could not evaluate"))
    }

    #[test]
    fn given_false_assignment_when_eval_then_false() {
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
