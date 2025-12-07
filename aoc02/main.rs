use std::collections::HashSet;
use std::{env, fs, process::ExitCode};

use iters::InvalidIdIterator;
use iters::RangesParser;
mod iters;

fn range_split((min, max): (u64, u64)) -> Vec<(u64, u64)> {
    // split ranges like 50-5000 into subranges 50-99, 100-999, 1000-5000
    let mut sub_ranges = Vec::<(u64, u64)>::new();
    let mut lower_bound = min;
    while lower_bound < max {
        let next_ten_power = u64::pow(10, lower_bound.checked_ilog10().unwrap_or(0) + 1);
        sub_ranges.push((lower_bound, std::cmp::min(next_ten_power - 1, max)));
        lower_bound = next_ten_power;
    }
    sub_ranges
}

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

        // first solution
        let invalid_id_iter = InvalidIdIterator::new(range.0, range.1, None);
        println!("{:?}", invalid_id_iter);
        for invalid_id in invalid_id_iter {
            println!("invalid id >>> {}", invalid_id);
            solution_part_one += invalid_id;
        }

        // second solution
        let sub_ranges = range_split(range);
        for srange in sub_ranges {
            println!("=========================================");
            println!("{}-{}", srange.0, srange.1);

            let mut invalid_id_set = HashSet::new();
            let min_digits: u32 = 1;
            let max_digits: u32 = 9;
            for prefix_digits_count in min_digits..=max_digits {
                let invalid_id_iter =
                    InvalidIdIterator::new(srange.0, srange.1, Some(prefix_digits_count));
                println!("{:?}", invalid_id_iter);
                for invalid_id in invalid_id_iter {
                    println!("invalid id >>> {}", invalid_id);
                    invalid_id_set.insert(invalid_id); // prevent duplicates within range
                }
            }
            for invalid_id in &invalid_id_set {
                solution_part_two += invalid_id;
            }
        }
    }

    println!("The sum of invalid IDs is: {}", solution_part_one);
    println!(
        "The sum of multi-pattern invalid IDs is: {}",
        solution_part_two
    );

    ExitCode::SUCCESS
}
