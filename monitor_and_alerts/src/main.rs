use crate::config::loader::ConfigLoader;

mod types;
mod config;

fn main() {
    let test = ConfigLoader::load_from_file("./config.toml");
    println!("{:?}", test);
}
