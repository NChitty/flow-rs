use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::str::FromStr;
use crate::bdd::{BDDError, BinaryDecisionDiagram, DecisionNode};
use crate::bdd::BDDError::ParseError;
use crate::bdd::BinaryNode::{Decision, Terminal};
use crate::Variable;

impl FromStr for BinaryDecisionDiagram {
    type Err = BDDError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let var_line = lines.next()
            .ok_or_else(|| ParseError("Variable line not present"))?
            .split_ascii_whitespace();
        let node_line = lines.next()
            .ok_or_else(|| ParseError("Node line not present"))?
            .split_ascii_whitespace();

        let num_vars = var_line.skip(1)
            .next()
            .ok_or_else(|| ParseError("Var line does not specify number"))?
            .parse::<usize>()?;
        let num_nodes = node_line.skip(1)
            .next()
            .ok_or_else(|| ParseError("Node line does not specify number"))?
            .parse::<usize>()?;

        let mut variables= HashMap::with_capacity(num_vars);
        let mut nodes = HashMap::with_capacity(num_nodes);

        for line in lines {
            let mut split = line.split_ascii_whitespace();
            let node_num = split.next()
                .ok_or_else(|| ParseError("Node num not present"))?
                .parse::<usize>()?;
            let node_if_true = split.next()
                .ok_or_else(|| ParseError("True Node number not present"))?
                .parse::<isize>()?;
            let node_if_false = split.next()
                .ok_or_else(|| ParseError("False Node number not present"))?
                .parse::<isize>()?;
            let var_id = split.next()
                .ok_or_else(|| ParseError("Var ID not present"))?
                .parse::<usize>()?;

            if node_if_true < 0 && node_if_false < 0 {
                nodes.insert(node_num, Terminal(var_id == 1));
                continue;
            }

            match variables.entry(var_id) {
                Entry::Vacant(v) => {
                    v.insert(Variable::new());
                },
                _ => {},
            }

            nodes.insert(node_num, Decision(
                DecisionNode::new_node(
                    node_if_false as usize,
                    node_if_true as usize,
                    var_id
                )));
        }

        Ok(Self {
            variables,
            nodes,
        })
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use crate::bdd::{BinaryDecisionDiagram, DecisionNode};
    use crate::bdd::BinaryNode::{Decision, Terminal};

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
}
