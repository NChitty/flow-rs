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

use std::fmt::Debug;
use std::io::Write;
use std::path::Path;
use std::{fs, io};

use clap::{ArgGroup, Args, Parser, Subcommand, ValueEnum};
use flow::bdd::BinaryDecisionDiagram;
use flow::{byte_to_bools, Evaluate, FlowError};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None, multicall = true)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(Debug, Subcommand)]
enum Action {
    /// load into memory
    Read(ReadArguments),
    /// evaluate logical artifact
    Evaluate(EvaluateArguments),
    /// exit the program
    Quit,
}

#[derive(Args, Debug)]
struct ReadArguments {
    /// The type of logical artifact to operate on
    #[arg(value_enum, required = true)]
    r#type: ArtifactType,
    /// The file to read from
    file: String,
}

#[derive(Args, Debug)]
#[command(group(ArgGroup::new("input").required(true).args(["hex", "bools"])))]
struct EvaluateArguments {
    /// hex string, must be an even number of characters
    #[arg(short = 'x', long, required_unless_present = "bools")]
    hex: Option<String>,

    /// Input as a sequence of bools, provided as comma-separated list
    #[arg(short, long, required_unless_present = "hex", value_delimiter = ' ', num_args = 1..)]
    bools: Option<Vec<bool>>,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum ArtifactType {
    /// apply action to a binary decision diagram
    #[value(name = "bdd")]
    BinaryDecisionDiagram,
    /// apply action to crossbar matrix
    #[value(name = "xbar")]
    CrossbarMatrix,
}

#[derive(Default)]
struct ApplicationContext {
    logical_artifact: Option<Box<dyn Evaluate>>,
}

fn main() -> Result<(), String> {
    let mut app_context: ApplicationContext = ApplicationContext::default();
    loop {
        let line = read_line()?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let cli = parse_command(line);
        match cli {
            Some(command) => {
                if respond(command, &mut app_context)? {
                    return Ok(());
                }
            },
            None => {
                continue;
            },
        }
    }
}

fn read_line() -> Result<String, String> {
    write!(io::stdout(), "$ ").map_err(|e| e.to_string())?;
    io::stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}

fn parse_command(line: &str) -> Option<Cli> {
    match Cli::try_parse_from(line.split_ascii_whitespace()) {
        Ok(cli) => Some(cli),
        Err(e) => {
            e.print().unwrap();
            None
        },
    }
}

fn respond(command: Cli, x: &mut ApplicationContext) -> Result<bool, String> {
    match command.action {
        Action::Read(args) => {
            let path = Path::new(args.file.as_str());
            let eval = match args.r#type {
                ArtifactType::BinaryDecisionDiagram => {
                    let bdd: BinaryDecisionDiagram = fs::read_to_string(path)
                        .map_err(|e| e.to_string())?
                        .parse()
                        .map_err(|e| match e {
                            FlowError::EvaluationError(str)
                            | FlowError::ParseError(str)
                            | FlowError::VariableAssignmentError(str) => str,
                        })?;
                    bdd
                },
                ArtifactType::CrossbarMatrix => {
                    todo!()
                },
            };
            x.logical_artifact = Some(Box::new(eval));

            Ok(false)
        },
        Action::Evaluate(args) => {
            let artifact = x
                .logical_artifact
                .as_mut()
                .ok_or("Must read in a logical artifact.")?;
            let bools: Vec<bool> = match args.hex {
                Some(hex) => (0..hex.len())
                    .step_by(2)
                    .map(|index| u8::from_str_radix(&hex[index..index + 2], 16))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|_| "Input must be a hex digit number.")?
                    .iter()
                    .flat_map(|byte| byte_to_bools(*byte))
                    .collect(),
                None => args.bools.unwrap(),
            };

            let bools = artifact.assign_vars(&bools).map_err(|e| match e {
                FlowError::EvaluationError(str)
                | FlowError::ParseError(str)
                | FlowError::VariableAssignmentError(str) => str,
            })?;

            let result = artifact.eval().map_err(|e| match e {
                FlowError::EvaluationError(str)
                | FlowError::ParseError(str)
                | FlowError::VariableAssignmentError(str) => str,
            })?;

            let output: String = bools
                .iter()
                .enumerate()
                .map(|(i, val)| format!("variable_{i} = {val}"))
                .fold(String::new(), |acc, x| format!("{acc}, {x}"));
            println!("{output}");
            println!("Evaluation: {result}");

            Ok(false)
        },
        Action::Quit => Ok(true),
    }
}

#[cfg(test)]
mod test {
    use clap::CommandFactory;

    use crate::Cli;

    #[test]
    fn verify_cmd() { Cli::command().debug_assert(); }
}
