extern crate yaml_rust;

use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;
use std::mem::transmute;
use self::yaml_rust::{YamlLoader, YamlEmitter, Yaml};

pub fn read_from_file(path: &str) -> Vec<Yaml> {
    let mut contents = String::new();

    File::open(path)
        .and_then(|mut file| file.read_to_string(&mut contents))
        .unwrap();

    return YamlLoader::load_from_str(&contents).unwrap();
}

pub fn write_to_file(path: &str, data: &Yaml) {
    let mut out_str = String::new();

    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(data).unwrap();
    }
    File::create(path)
        .and_then(|mut file| file.write_all(out_str.as_bytes()))
        .unwrap();
}

pub fn merge(from: &Yaml, to: &mut Yaml) {
    match *from {
        Yaml::Array(ref arr) => {
            for v in arr {
            }
        }

        Yaml::Hash(ref hash) => {
            for (k, v) in hash {
            }
        }

        _ => {
            *to = from.clone();
        }
    }
}