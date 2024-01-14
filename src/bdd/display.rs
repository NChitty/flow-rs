/*
 * Copyright (c) 2024 William Nicholas Chitty
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::fmt::{Display, Formatter};

use crate::bdd::{BinaryDecisionDiagram, BinaryNode, DecisionNode};

impl Display for BinaryDecisionDiagram {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let num_vars = self.variables.keys().len();
        let num_nodes = self.nodes.keys().len();
        writeln!(f, "vars {num_vars}")?;
        write!(f, "nodes {num_nodes}")?;
        for mapping in &self.nodes {
            write!(f, "\n{} {}", mapping.0, mapping.1)?;
        }
        write!(f, "")
    }
}

impl Display for BinaryNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryNode::Decision(node) => write!(f, "{node}"),
            BinaryNode::Terminal(true) => write!(f, "-1 -1 1"),
            BinaryNode::Terminal(false) => write!(f, "-1 -1 0"),
        }
    }
}

impl Display for DecisionNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.decision_map[1], self.decision_map[0], self.variable_id
        )
    }
}

#[cfg(test)]
mod test {
    use crate::bdd::{BinaryDecisionDiagram, BinaryNode, DecisionNode};

    #[test]
    fn display_decision_node() {
        let node = DecisionNode {
            variable_id: 3,
            decision_map: [2, 1],
        };
        assert_eq!(format!("{node}"), "1 2 3");
    }

    #[test]
    fn display_binary_node_decision() {
        let node = DecisionNode {
            variable_id: 3,
            decision_map: [2, 1],
        };
        let binary_node = BinaryNode::Decision(node);
        assert_eq!(format!("{binary_node}"), "1 2 3");
    }

    #[test]
    fn display_binary_node_terminal_true() {
        let binary_node = BinaryNode::Terminal(true);
        assert_eq!(format!("{binary_node}"), "-1 -1 1");
    }

    #[test]
    fn display_binary_node_terminal_false() {
        let binary_node = BinaryNode::Terminal(false);
        assert_eq!(format!("{binary_node}"), "-1 -1 0");
    }

    const SIMPLE_BDD: &str = "vars 1
nodes 3
0 1 2 0
1 -1 -1 0
2 -1 -1 1";

    #[test]
    fn display_bdd() {
        let bdd: BinaryDecisionDiagram = SIMPLE_BDD.parse().unwrap();
        let bdd_display = format!("{bdd}");

        assert_eq!(SIMPLE_BDD.len(), bdd_display.len());
        let simple_bdd_lines_vec: Vec<&str> = SIMPLE_BDD.lines().collect();
        let bdd_lines = bdd_display.lines();
        for line in bdd_lines {
            assert!(simple_bdd_lines_vec.contains(&line));
        }
    }
}
