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

use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct BinaryDecisionDiagram {
    variables: usize,
    nodes: HashMap<usize, BinaryNode>,
    entry_node: usize,
}

mod display;
mod eval;
mod parse;

#[derive(Debug, PartialEq)]
enum BinaryNode {
    Decision(DecisionNode),
    Terminal(bool),
}

#[derive(Debug, PartialEq)]
struct DecisionNode {
    pub variable_id: usize,
    decision_map: (usize, usize),
}

impl DecisionNode {
    pub fn new_node(node_if_false: usize, node_if_true: usize, variable_id: usize) -> Self {
        Self {
            variable_id,
            decision_map: (node_if_false, node_if_true),
        }
    }

    pub fn evaluate(&self, variable: bool) -> usize {
        if variable {
            self.decision_map.1
        } else {
            self.decision_map.0
        }
    }
}

#[cfg(test)]
mod test {
    use crate::bdd::DecisionNode;

    #[test]
    fn false_variable() {
        let node = DecisionNode::new_node(1, 0, 0);

        assert_eq!(1, node.evaluate(false));
    }

    #[test]
    fn true_variable() {
        let node = DecisionNode::new_node(0, 1, 0);

        assert_eq!(1, node.evaluate(true));
    }
}
