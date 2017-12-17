extern crate clap;
extern crate yaml_rust;

mod cli;
mod yaml;

use yaml_rust::Yaml;

fn main() {
    let matches = cli::build_cli().get_matches();
    let mut data = Yaml::Null;

    if let Some(inputs) = matches.values_of("input") {
        for input in inputs {
            let docs = yaml::read_from_file(input);
            for doc in &docs {
                yaml::merge(doc, &mut data);
            }
        }
    }

    if let Some(output) = matches.value_of("output") {
        yaml::write_to_file(output, &data);
    }
}