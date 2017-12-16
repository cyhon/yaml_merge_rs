extern crate clap;

mod cli;
mod yaml;

fn main() {
    let matches = cli::build_cli().get_matches();

    if let Some(inputs) = matches.values_of("input") {
        for input in inputs {
            let y = yaml::read_from_file(input);
            if let Some(output) = matches.value_of("output") {
                yaml::write_to_file(output, &y[0]);
            }
        }
    }
}