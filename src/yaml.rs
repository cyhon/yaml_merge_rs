extern crate yaml_rust;

use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;
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
        Yaml::Array(ref from_arr) => {
            match *to {
                Yaml::Array(ref mut to_arr) => {
                    for v in from_arr {
                        to_arr.push(v.clone());
                    }
                }

                Yaml::Null => {
                    *to = from.clone();
                }

                _ => {
                    panic!("yaml文件格式不兼容，无法合并")
                }
            }
        }

        Yaml::Hash(ref from_hash) => {
            match *to {
                Yaml::Hash(ref mut to_hash) => {
                    for (k, v) in from_hash {
                        {
                            match to_hash.get_mut(k) {
                                Some(to) => {
                                    merge(v, to);
                                    continue;
                                }
                                None => {}
                            }
                        }

                        to_hash.insert(k.clone(), v.clone());
                    }
                }

                Yaml::Null => {
                    *to = from.clone();
                }

                _ => {
                    panic!("yaml文件格式不兼容，无法合并")
                }
            }
        }

        _ => {
            *to = from.clone();
        }
    }
}