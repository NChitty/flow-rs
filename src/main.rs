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

use clap::{Args, Parser, ValueEnum};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The type of logical artifact to operate on
    #[arg(value_enum, required = true)]
    r#type: ArtifactType,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ArtifactType {
    /// apply action to a binary decision diagram
    #[value(name = "bdd")]
    BinaryDecisionDiagram,
    /// apply action to crossbar matrix
    #[value(name = "xbar")]
    CrossbarMatrix
}

fn main() {
    let cli = Cli::parse();

    match cli.r#type {
        ArtifactType::BinaryDecisionDiagram => println!("Operating on bdd."),
        ArtifactType::CrossbarMatrix => println!("Operating on crossbar.")
    }
}
