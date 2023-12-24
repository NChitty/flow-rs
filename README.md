Welcome to Flow-rs. This project was originally written in Python as a project for school. The
goal of the project is to provide a command-line utility for reading, evaluating and even 
synthesizing binary decision diagrams and crossbar matrices.

# Binary Decision Diagrams
The representation for a binary decision diagram is the following:
```
vars <num_vars>
nodes <num_nodes>
node_id node_if_true node_if_false variable_id
...
node_id -1 -1 0 // reaching this node gives a false evaluation
node_id -1 -1 1 // reaching this node gives a true evaluation
```
The `node_id` is how the internal systems will know which node to go to when evaluating. For 
example, in this SIMPLE_BDD:
```
vars 1
nodes 3
0 1 2 0
1 -1 -1 1
2 -1 -1 0
```
When variable `0` is true the BDD will evaluate to `true`. When variable `1` is false the BDD 
will evaluate to `false` because the BDD starts at the top node, evaluates the variable with the 
given ID, and traverses to the node accordingly.

# Crossbar Matrix
TODO
