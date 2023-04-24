// Author: Dr. Mo Ashouri
// Email: ashourics@gmail.com
// License: MIT

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;
use csv::Writer;

fn main() -> Result<(), Box<dyn Error>> {
    // Read command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        std::process::exit(1);
    }
    let input_file = &args[1];
    let output_file = &args[2];

    // Open input file
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    // Create output CSV file
    let mut writer = Writer::from_path(output_file)?;

    // Process each line in the input file
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
        if parts.len() != 2 {
            eprintln!("Invalid line format: {}", line);
            continue;
        }

        let word = parts[0];
        let meaning = parts[1];
        writer.write_record(&[word, meaning])?;
    }

    writer.flush()?;

    println!("Anki dictionary file created: {}", output_file);
    Ok(())
}
