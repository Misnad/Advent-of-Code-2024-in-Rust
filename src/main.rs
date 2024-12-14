mod advent1;
mod advent2;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a day number (e.g., 1, 2, ...).");
        return;
    }

    match args[1].as_str() {
        "1" => advent1::solve(),
        "2" => advent2::solve(),
        // Add more cases for additional days
        _ => eprintln!("Invalid day number. Please provide a valid day (1, 2, ...)."),
    }
}
