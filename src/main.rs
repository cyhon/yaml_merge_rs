extern crate yaml_rust;

mod cli;
mod yaml;

use yaml_rust::Yaml;

fn main() {
    let matches = cli::build_cli().get_matches();

    let inputs = matches.values_of_lossy("input").unwrap();
    let output_path = matches.value_of("output").unwrap();

    let input_yaml_docs: Vec<Yaml> = inputs
        .iter()
        .flat_map(|path| yaml::read_from_file(path))
        .collect();

    yaml::write_to_file(output_path, yaml::merge_all(input_yaml_docs));
}