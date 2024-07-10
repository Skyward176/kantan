use std::io;
use std::fs;
use yaml_rust2::YamlLoader;

fn main() {
    println!("Hello, world!");
    build_fs("data/schema.yaml");
}
fn build_fs(file: &str) {// takes a schema as yaml and creates the directories from it
    let contents: String = fs::read_to_string(file).expect("Could not read file");
    let schema = YamlLoader::load_from_str(&contents).unwrap();

    println!("Schema loaded!");
    fs::create_dir_all("./test/hi");//use this to make sure whole directory tree is created.
    println!("Called create dir!");
}
