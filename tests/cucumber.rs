use std::str::FromStr;

use cucumber::gherkin::Step;
use cucumber::{given, then, when, Parameter, World};
use flow::bdd::BinaryDecisionDiagram;
use flow::{byte_to_bools, Evaluate};

#[derive(Debug, Default, World)]
pub struct BddWorld {
    bdd: BinaryDecisionDiagram,
}

#[derive(Debug, Default, Eq, Parameter, PartialEq)]
#[param(name = "vars", regex = "[0-9a-fA-F]*")]
struct Variables {
    variables: Vec<bool>,
}

impl FromStr for Variables {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let variables: Vec<bool> = (0..s.len())
            .step_by(2)
            .map(|index| u8::from_str_radix(&s[index..index + 2], 16))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "Input must be a hex digit number.")?
            .iter()
            .flat_map(|byte| byte_to_bools(*byte))
            .collect();
        Ok(Self { variables })
    }
}

#[given("a bdd with definition")]
fn parse_bdd(world: &mut BddWorld, step: &Step) {
    let definition = step.docstring().expect("Docstring not present.");
    world.bdd = definition
        .trim()
        .parse()
        .expect("Could not parse docstring.");
}

#[when(expr = "{vars} is assigned as hex")]
fn assign_var(world: &mut BddWorld, vars: Variables) {
    let _ = world.bdd.assign_vars(&vars.variables);
}

#[then(expr = "the evaluation should be {word}")]
fn evaluate(world: &mut BddWorld, expect: bool) {
    assert_eq!(expect, world.bdd.eval().expect("Could not evaluate BDD."))
}

fn main() { futures::executor::block_on(BddWorld::run("tests/features/bdd.feature")); }
