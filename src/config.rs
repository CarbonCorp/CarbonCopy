extern crate yaml_rust;
use std::fs;

pub fn get_config() -> yaml_rust::Yaml {
    let config = fs::read_to_string("Carbonfile").expect("unable to read Carbonfile");
    let docs = yaml_rust::YamlLoader::load_from_str(&config).unwrap();
    docs[0].clone()
}
