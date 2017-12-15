extern crate clap;

mod cli;

fn main() {
    let matches = cli::build_cli().get_matches();

    if let Some(inputs) = matches.values_of("input") {
        for input in inputs {
            println!("Value of input: {}", input);
        }
    }
    if let Some(output) = matches.value_of("output") {
        println!("Value of output: {}", output);
    }
}