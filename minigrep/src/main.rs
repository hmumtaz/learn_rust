use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|e| {
        eprintln!("Problem parsing args: {}", e);
        process::exit(1)
    });
    dbg!(format!(
        "Searching for {} in {}",
        &config.query, &config.filename
    ));

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}
