use crate::bdd::DecisionNode;
use crate::Variable;

#[test]
fn given_unassigned_variable_then_error() {
    let var = Variable {
        value: None,
    };
    let node = DecisionNode::new_node(0, 0, 0);

    assert!(node.evaluate(&var).err().is_some());
}

#[test]
fn given_false_variable_then_false_node() {
    let var = Variable {
        value: Some(false),
    };
    let node = DecisionNode::new_node(1, 0, 0);

    assert_eq!(1, node.evaluate(&var).unwrap());
}

#[test]
fn given_true_variable_then_true_node() {
    let var = Variable {
        value: Some(true),
    };
    let node = DecisionNode::new_node(0, 1, 0);

    assert_eq!(1, node.evaluate(&var).unwrap());
}
