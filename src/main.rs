use std::env::args;

use arkrail_guardian::{config::create::create_config, emulator::mumu::MuMu};
fn main() {
    run();
}

fn run() {
    create_config();
    let args: Vec<String> = args().collect();
    match MuMu::new().open_instance(&args[1]) {
        Err(e) => {
            eprintln!("Failed to kill instance: {}", e);
        }
        Ok(_) => {
            println!("Instance killed successfully.");
        }
    }
}
