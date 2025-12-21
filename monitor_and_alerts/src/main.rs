use crate::config::loader::ConfigLoader;

mod config;
mod types;
mod errors;

fn main() {
    let test = ConfigLoader::load_from_file("./config.toml");
    println!("{:?}", test);
}
