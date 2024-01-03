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

use std::io;
use std::io::Write;
use clap::{ArgMatches, Args, CommandFactory, Error, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None, multicall = true)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(Debug, Subcommand)]
enum Action {
    /// load into memory
    Read(Arguments),
    /// exit the program
    Quit
}

#[derive(Args, Debug)]
struct Arguments {
    /// The type of logical artifact to operate on
    #[arg(value_enum, required = true)]
    r#type: ArtifactType,
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

fn main() -> Result<(), String> {
    loop {
        let line = read_line()?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let cli = parse_command(line);
        match cli {
            Some(command) => if respond(command)? {
                return Ok(());
            },
            None => {
                continue;
            }
        }
    }
}

fn respond(command: Cli) -> Result<bool, String> {
    match command.action {
        Action::Read(_) => {
            println!("TODO");
            Ok(false)
        },
        Action::Quit => Ok(true)
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
        }
    }
}
