/*
 *    Copyright (c) 2023 William Nicholas Chitty
 *
 *    Licensed under the Apache License, Version 2.0 (the "License");
 *    you may not use this file except in compliance with the License.
 *    You may obtain a copy of the License at
 *
 *        http://www.apache.org/licenses/LICENSE-2.0
 *
 *    Unless required by applicable law or agreed to in writing, software
 *    distributed under the License is distributed on an "AS IS" BASIS,
 *    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *    See the License for the specific language governing permissions and
 *    limitations under the License.
 */

use crate::bdd::BDDError::ParseError;
use crate::bdd::BinaryNode::{Decision, Terminal};
use crate::bdd::{BDDError, BinaryDecisionDiagram, DecisionNode};
use crate::Variable;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::str::FromStr;

impl FromStr for BinaryDecisionDiagram {
    type Err = BDDError;

    #[allow(clippy::cast_sign_loss)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let mut var_line = lines
            .next()
            .ok_or(ParseError("Variable line not present"))?
            .split_ascii_whitespace();
        let mut node_line = lines
            .next()
            .ok_or(ParseError("Node line not present"))?
            .split_ascii_whitespace();

        let num_vars = var_line
            .nth(1)
            .ok_or(ParseError("Var line does not specify number"))?
            .parse::<usize>()?;
        let num_nodes = node_line
            .nth(1)
            .ok_or(ParseError("Node line does not specify number"))?
            .parse::<usize>()?;

        let mut variables = HashMap::with_capacity(num_vars);
        let mut nodes = HashMap::with_capacity(num_nodes);
        let mut entry_node: Option<usize> = None;
        for line in lines {
            let mut split = line.split_ascii_whitespace();
            let node_num = split
                .next()
                .ok_or(ParseError("Node num not present"))?
                .parse::<usize>()?;
            let node_if_true = split
                .next()
                .ok_or(ParseError("True Node number not present"))?
                .parse::<isize>()?;
            let node_if_false = split
                .next()
                .ok_or(ParseError("False Node number not present"))?
                .parse::<isize>()?;
            let var_id = split
                .next()
                .ok_or(ParseError("Var ID not present"))?
                .parse::<usize>()?;

            if node_if_true < 0 && node_if_false < 0 {
                nodes.insert(node_num, Terminal(var_id == 1));
                continue;
            }

            if entry_node.is_none() {
                entry_node = Some(node_num);
            }

            if let Entry::Vacant(v) = variables.entry(var_id) {
                v.insert(Variable::new());
            }

            nodes.insert(
                node_num,
                Decision(DecisionNode::new_node(
                    node_if_false as usize,
                    node_if_true as usize,
                    var_id,
                )),
            );
        }

        if num_vars != variables.len() || num_nodes != nodes.len() {
            return Err(ParseError("Number of tokens does not match first lines"));
        }

        let mut has_false = false;
        let mut has_true = false;
        nodes
            .values()
            .filter(|&node| match node {
                Decision(_) => false,
                Terminal(_) => true,
            })
            .for_each(|terminal| match terminal {
                Terminal(val) => {
                    if *val {
                        has_true = true;
                    } else {
                        has_false = true;
                    }
                }
                Decision(_) => panic!("How did you get here?"),
            });

        if !(has_true && has_false) {
            return Err(ParseError("Not both types of terminal nodes."));
        }

        Ok(Self {
            variables,
            nodes,
            entry_node: entry_node.ok_or(ParseError("No entry node was set"))?,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::bdd::BinaryNode::{Decision, Terminal};
    use crate::bdd::{BinaryDecisionDiagram, DecisionNode};
    use std::str::FromStr;

    const FREE_BDD_2: &str = "vars 2
nodes 4
1 4 2 1
2 4 3 2
3 -1 -1 0
4 -1 -1 1";

    #[test]
    fn from_string() {
        let bdd = BinaryDecisionDiagram::from_str(FREE_BDD_2).unwrap();

        assert_eq!(2, bdd.variables.len());
        assert_eq!(4, bdd.nodes.len());
        assert_eq!(
            &Decision(DecisionNode::new_node(2, 4, 1)),
            bdd.nodes.get(&1).unwrap()
        );
        assert_eq!(
            &Decision(DecisionNode::new_node(3, 4, 2)),
            bdd.nodes.get(&2).unwrap()
        );
        assert_eq!(&Terminal(false), bdd.nodes.get(&3).unwrap());
        assert_eq!(&Terminal(true), bdd.nodes.get(&4).unwrap());
    }

    #[test]
    fn given_empty_string_then_error() {
        let bdd = BinaryDecisionDiagram::from_str("");
        assert!(bdd.is_err());
    }

    #[test]
    fn given_var_line_only_then_error() {
        let bdd = BinaryDecisionDiagram::from_str("vars 1");
        assert!(bdd.is_err());
    }

    #[test]
    fn given_nodes_line_only_then_error() {
        let bdd = BinaryDecisionDiagram::from_str(
            "vars 1
nodes 1",
        );
        assert!(bdd.is_err());
    }

    #[test]
    fn given_negative_node_id_then_error() {
        let bdd = BinaryDecisionDiagram::from_str(
            "vars 1
nodes 1
-1 0 0 0",
        );
        assert!(bdd.is_err());
    }

    #[test]
    fn given_non_matching_vars_then_error() {
        let bdd = BinaryDecisionDiagram::from_str(
            "vars 2
nodes 1
0 0 0 0",
        );
        assert!(bdd.is_err());
    }

    #[test]
    fn given_non_matching_nodes_then_error() {
        let bdd = BinaryDecisionDiagram::from_str(
            "vars 1
nodes 2
0 0 0 0",
        );
        assert!(bdd.is_err());
    }

    #[test]
    fn given_no_terminal_nodes_then_error() {
        let bdd = BinaryDecisionDiagram::from_str(
            "vars 1
nodes 1
0 0 0 0",
        );
        assert!(bdd.is_err());
    }

    #[test]
    fn given_only_true_terminal_nodes_then_error() {
        let bdd = BinaryDecisionDiagram::from_str(
            "vars 1
nodes 2
0 1 0 0
1 -1 -1 1",
        );
        assert!(bdd.is_err());
    }

    #[test]
    fn given_only_false_terminal_nodes_then_error() {
        let bdd = BinaryDecisionDiagram::from_str(
            "vars 1
nodes 2
0 1 2 0
2 -1 -1 0",
        );
        assert!(bdd.is_err());
    }

    #[test]
    fn given_parseable_then_ok() {
        let bdd = BinaryDecisionDiagram::from_str(
            "vars 1
nodes 3
0 1 2 0
1 -1 -1 0
2 -1 -1 1",
        );
        assert!(bdd.is_ok());
    }
}
