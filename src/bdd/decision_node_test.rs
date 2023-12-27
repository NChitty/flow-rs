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

use crate::bdd::DecisionNode;
use crate::Variable;

#[test]
fn given_unassigned_variable_then_error() {
    let var = Variable { value: None };
    let node = DecisionNode::new_node(0, 0, 0);

    assert!(node.evaluate(&var).err().is_some());
}

#[test]
fn given_false_variable_then_false_node() {
    let var = Variable { value: Some(false) };
    let node = DecisionNode::new_node(1, 0, 0);

    assert_eq!(1, node.evaluate(&var).unwrap());
}

#[test]
fn given_true_variable_then_true_node() {
    let var = Variable { value: Some(true) };
    let node = DecisionNode::new_node(0, 1, 0);

    assert_eq!(1, node.evaluate(&var).unwrap());
}
