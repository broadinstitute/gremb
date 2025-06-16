
fn main() {
    match gremb::run() {
        Ok(_) => { println!("Done!") }
        Err(error) => { eprintln!("Error: {}", error) }
    }
}
