use std::collections::HashSet;
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
    let mut solution_part_one: u64 = 0;
    let mut solution_part_two: u64 = 0;
    for range in RangesParser::new(&input) {
        println!("{}-{}", range.0, range.1);

        let invalid_id_iter = InvalidIdIterator::new(range.0, range.1, None);
        println!("{:?}", invalid_id_iter);
        for invalid_id in invalid_id_iter {
            println!("{}", invalid_id);
            solution_part_one += invalid_id;
        }

        println!("=========================================");

        let mut invalid_id_set = HashSet::new();
        let min_digits: u32 = 0;
        let max_digits: u32 = 9;
        for prefix_digits_count in min_digits..=max_digits {
            let invalid_id_iter =
                InvalidIdIterator::new(range.0, range.1, Some(prefix_digits_count));
            println!("{:?}", invalid_id_iter);
            for invalid_id in invalid_id_iter {
                println!("{}", invalid_id);
                invalid_id_set.insert(invalid_id);
            }
        }
        for invalid_id in &invalid_id_set {
            solution_part_two += invalid_id;
        }
    }

    println!("The sum of invalid IDs is: {}", solution_part_one);
    println!(
        "The sum of multi-pattern invalid IDs is: {}",
        solution_part_two
    );

    ExitCode::SUCCESS
}
