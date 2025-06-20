use log::LevelFilter;
use simplelog::{Config, SimpleLogger};

fn main() {
    SimpleLogger::init(LevelFilter::Info, Config::default()).unwrap();
    match gremb::run() {
        Ok(_) => { println!("Done!") }
        Err(error) => { eprintln!("Error: {}", error) }
    }
}
