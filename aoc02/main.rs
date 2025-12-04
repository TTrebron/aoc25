use std::{env, fs, process::ExitCode};

use iters::InvalidIdIterator;
use iters::RangesParser;
mod iters;

fn main() -> ExitCode {
    // get filename from first param
    let filename;
    match env::args().nth(1) {
        Some(arg) => filename = arg,
        None => {
            eprintln!(
                "Usage: {} <inputfile>",
                env::args().nth(0).unwrap_or_default()
            );
            return ExitCode::FAILURE;
        }
    }

    // read file contents into a string
    let input;
    match fs::read_to_string(&filename) {
        Ok(contents) => input = contents,
        Err(e) => {
            eprintln!("Error reading {}: {}", filename, e);
            return ExitCode::FAILURE;
        }
    }

    // parse ranges and iterate through all invalid ids
    let mut solution: u64 = 0;
    for range in RangesParser::new(&input) {
        println!("{}-{}", range.0, range.1);
        let invalid_id_iter = InvalidIdIterator::new(range.0, range.1, None);
        println!("{:?}", invalid_id_iter);
        for invalid_id in invalid_id_iter {
            //println!("{}", invalid_id);
            solution += invalid_id;
        }
    }

    println!("The sum of invalid IDs is: {}", solution);

    ExitCode::SUCCESS
}
