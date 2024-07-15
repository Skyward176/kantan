use std::fs::File;
use std::fs::create_dir_all;
use std::io::prelude::*;
use yaml_rust2 as yaml;

fn main() {
    println!("Hello, world!");
    read_schema("data/schema.yaml", "scratch");
}
fn generate_paths(base: &str, path: &str) {
    let full_path :String = format!("./{}/{}", base, path);
    create_dir_all(full_path);// need to handle errors here
}
fn read_schema(file: &str, dest: &str) {// takes a schema as yaml and creates the directories from it
    println!("Trying to open {}", file);
    let mut contents = File::open(file).unwrap();
    let mut string = String::new();
    contents.read_to_string(&mut string).unwrap();

    let doc = yaml::YamlLoader::load_from_str(&string).unwrap();
    for item in doc[0]["directories"].clone() {
        generate_paths(dest, item.as_str().unwrap());
    }
}
