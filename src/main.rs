use std::io;
use std::fs;

fn main() {
    println!("Hello, world!");
    build_fs();
}
fn build_fs() {// takes a schema as json and creates the directories from it
    fs::create_dir_all("./test/hi");//use this to make sure whole directory tree is created.
    println!("Called create dir!");
}
