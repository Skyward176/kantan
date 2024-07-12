use std::fs::File;
use std::io::prelude::*;
use fstrings::f;
use yaml_rust2 as yaml;

fn main() {
    println!("Hello, world!");
    build_fs("data/schema.yaml");
}
fn generate_path(path: &str) -> String {
    let path = f!("{}/{}", "scratch", "test");
    println!("{}", path);
}
fn build_fs(file: &str) {// takes a schema as yaml and creates the directories from it
    println!("Trying to open {}", file);
    let mut contents = File::open(file).unwrap();
    let mut string = String::new();
    contents.read_to_string(&mut string).unwrap();

    let doc = yaml::YamlLoader::load_from_str(&string).unwrap();
    for item in doc[0]["directories"].clone() {
        println!("{:?}", item);
    }
    
    println!("Called create dir!");
}
