use std::fs::File;
use std::fs::create_dir_all;
use std::io;
use std::io::Read;
use yaml_rust2 as yaml;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    init {
        /// lists test values
        // Optional schema selection
        #[arg(short, long)]
        schema: Option<String>,
        // Optional target selection
        #[arg(short, long)]
        target: Option<String>,
    },
}
fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::init {schema, target}) => {
            println!("Init command");
            init()
        }
        None => {}
    }

}
fn init() {
    while true {
        println!("Enter the path to the schema file");
        let mut file = String::new();
        io::stdin().read_line(&mut file).expect("Failed to read line");
        println!("Enter the destination directory");
        let mut dest = String::new();
        io::stdin().read_line(&mut dest).expect("Failed to read line");
        read_schema(file.trim(), dest.trim());
        println!("Init completed!");
        break;
    }

}
fn generate_paths(base: &str, path: &str) {
    let full_path :String = format!("./{}/{}", base, path);
    create_dir_all(full_path);// need to handle errors here
}
fn read_schema(file: &str, dest: &str) {// takes a schema as yaml and creates the directories from it
    let mut contents = File::open(file).unwrap();
    let mut string = String::new();
    contents.read_to_string(&mut string).unwrap();

    let doc = yaml::YamlLoader::load_from_str(&string).unwrap();
    for item in doc[0]["directories"].clone() {
        generate_paths(dest, item.as_str().unwrap());
    }
}
