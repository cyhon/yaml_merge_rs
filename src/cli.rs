extern crate clap;
use self::clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new("yaml_merge")
        .version("0.1.0")
        .author("hetiu.")
        .about("Merge some yaml files into one!")
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .required(true)
            .multiple(true)
            .takes_value(true)
            .value_name("example.yml")
            .help("Input yaml files"))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .required(true)
            .takes_value(true)
            .value_name("out.yml")
            .help("Output yaml file"))
}