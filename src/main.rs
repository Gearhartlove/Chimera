use core::fmt;
use std::{collections::HashSet, error::Error, ffi::OsString, fs};

use clap::{arg, Command};

fn parse() {
    let matches = Command::new("Chimera")
        .version("1.0")
        .about("The file headed monster.")
        .arg(arg!(--input <VALUE>).required(true))
        .arg(arg!(--output <VALUE>).required(true))
        .get_matches();

    println!(
        "input: {:?}",
        matches.get_one::<String>("input").expect("required")
    );

    println!(
        "output: {:?}",
        matches.get_one::<String>("output").expect("required")
    )
}

fn source() -> String {
    r#"""
from pprint import pprint
dict = {"foo": "bar"}
pprint(dict)
"""#
    .to_string()
}

fn source2() -> String {
    r#"""
import dataclases

def my_func():
    pass
"""#
    .to_string()
}

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum ChimeraError {
    InputNotFound(String),
    ProblemAccesingFile(String),
    ProblemConvertingFileNameToString(OsString),
    MultipleSources(Vec<String>),
    UnsupportedSource,
    UnsupportedEngine,
    UndeterminedSource,
}

impl fmt::Display for ChimeraError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChimeraError::MultipleSources(sources) => {
                write!(f, "Multiple sources found. sources=[{:?}]", sources)
            }
            ChimeraError::UnsupportedSource => todo!(),
            _ => todo!(),
        }
    }
}

impl Error for ChimeraError {}

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum SupportedSource {
    Python,
    Unknown,
}

impl SupportedSource {
    fn from_extension(extension: &str) -> Result<SupportedSource, ChimeraError> {
        match extension {
            "py" => Ok(SupportedSource::Python),
            _ => Err(ChimeraError::UnsupportedSource),
        }
    }
}

fn determine_source_kind(sources: &Vec<String>) -> Result<SupportedSource, ChimeraError> {
    let extensions = sources
        .iter()
        .flat_map(|s| s.split(".").last())
        .map(|s| SupportedSource::from_extension(s))
        .collect::<HashSet<_>>();

    match extensions.len() {
        0 => Err(ChimeraError::UndeterminedSource),
        1 => {
            let extension = extensions.into_iter().next().unwrap();
            extension
        }
        _ => Err(ChimeraError::MultipleSources(sources.to_vec())),
    }
}

struct ChimeraEngine {}
impl ChimeraEngine {
    fn combine(sources: Vec<String>, source_kind: SupportedSource) -> Result<String, ChimeraError> {
        match source_kind {
            SupportedSource::Python => ChimeraEngine::combine_python(sources),
            _ => Err(ChimeraError::UnsupportedEngine),
        }
    }

    fn combine_python(sources: Vec<String>) -> Result<String, ChimeraError> {
        // to start, just concatenate the files together and return that
        // read the
        todo!()
    }
}

fn determine_sources(directory: String) -> Result<Vec<String>, ChimeraError> {
    let paths = fs::read_dir(format!("{}", directory)).map_err(|e| {
        ChimeraError::InputNotFound(format!("Failed to find input directory: {}", e))
    })?;
    let mut file_names: Vec<String> = vec![];
    for path in paths {
        let entry = path.map_err(|e| {
            let message = format!("Problem accessing file: {}", e);
            ChimeraError::ProblemAccesingFile(message)
        })?;
        let name =
            entry
                .file_name()
                .to_str()
                .ok_or(ChimeraError::ProblemConvertingFileNameToString(
                    entry.file_name(),
                ))?
                .to_string();

        file_names.push(name);
    }

    Ok(file_names)
}

pub fn main() -> Result<(), ChimeraError> {
    let directory = "/home/kristoff/tsl/projects/rust/chimera/examples/python_files".to_string();
    let sources = determine_sources(directory)?;
    let source_kind = determine_source_kind(sources.as_ref())?;
    let combination_result = ChimeraEngine::combine(sources, source_kind);
    // save the file to the output directory
    Ok(())
}
