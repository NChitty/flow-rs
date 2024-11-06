use std::str::FromStr;

use cucumber::gherkin::Step;
use cucumber::{given, then, when, Parameter, World};
use flow::bdd::BinaryDecisionDiagram;
use flow::{byte_to_bools, Evaluate};

#[derive(Debug)]
enum Artifact {
    Bdd(BinaryDecisionDiagram),
}

impl Default for Artifact {
    fn default() -> Self { Self::Bdd(BinaryDecisionDiagram::default()) }
}

#[derive(Debug, Default, World)]
pub struct FlowWorld {
    artifact: Artifact,
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
fn parse_bdd(world: &mut FlowWorld, step: &Step) {
    let definition = step.docstring().expect("Docstring not present.");
    world.artifact = Artifact::Bdd(
        definition
            .trim()
            .parse()
            .expect("Could not parse docstring."),
    );
}

#[when(expr = "{vars} is assigned as hex")]
fn assign_var(world: &mut FlowWorld, vars: Variables) -> Result<(), String> {
    let _ = match world.artifact {
        Artifact::Bdd(ref mut bdd) => {
            bdd.assign_vars(&vars.variables)
                .map_err(|err| err.to_string())?;
        },
    };
    Ok(())
}

#[then(expr = "the evaluation should be {word}")]
fn evaluate(world: &mut FlowWorld, expect: bool) -> Result<(), String> {
    let actual = match world.artifact {
        Artifact::Bdd(ref bdd) => bdd.eval().map_err(|err| err.to_string())?,
    };

    assert_eq!(expect, actual);
    Ok(())
}

#[then("the truth table should equal")]
fn truth_table(world: &mut FlowWorld, step: &Step) -> Result<(), String> {
    let truth_table = match world.artifact {
        Artifact::Bdd(ref mut bdd) => bdd
            .truth_table()
            .map_err(|err| err.to_string())?
            .iter()
            .enumerate()
            .map(|(i, val)| format!("{i:x} = {val}"))
            .collect::<Vec<_>>()
            .join("\n"),
    };
    assert_eq!(
        step.docstring().expect("Docstring not present.").trim(),
        truth_table
    );
    Ok(())
}

fn main() { futures::executor::block_on(FlowWorld::run("tests/features/bdd.feature")); }
