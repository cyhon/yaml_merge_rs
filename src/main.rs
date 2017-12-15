extern crate clap;
extern crate yaml_rust;

mod cli;

use std::fs::File;
use std::io::prelude::*;
use std::collections::BTreeMap;
use yaml_rust::{YamlLoader, YamlEmitter, ScanError};

fn main() {
    let matches = cli::build_cli().get_matches();

    if let Some(inputs) = matches.values_of("input") {
        let mut contents = String::new();

        for input in inputs {
            File::open(input)
                .and_then(|mut file| file.read_to_string(&mut contents))
                .unwrap();

            let yaml = YamlLoader::load_from_str(&contents).unwrap();

            println!("input: {:?}", yaml)
        }
    }

    if let Some(output) = matches.value_of("output") {
        println!("Value of output: {}", output);
    }
}