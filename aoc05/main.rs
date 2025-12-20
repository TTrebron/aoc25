use std::{
    collections::BTreeMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
    process::ExitCode,
};

fn main() -> ExitCode {
    let filename;
    match env::args().nth(1) {
        Some(arg) => filename = arg,
        None => {
            eprintln!(
                "Usage: {} <filename>",
                env::args().nth(0).unwrap_or_default()
            );
            return ExitCode::FAILURE;
        }
    }

    let reader;
    match File::open(filename) {
        Ok(file) => reader = BufReader::new(file),
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return ExitCode::FAILURE;
        }
    }

    let mut ranges: BTreeMap<usize, usize> = BTreeMap::new();

    let mut lines_iter = reader.lines();
    loop {
        let Some(line_res) = lines_iter.next() else {
            return ExitCode::SUCCESS; // EOF
        };

        match line_res {
            Ok(current_line) => {
                let current_line_trimmed = current_line.trim();

                if current_line_trimmed.is_empty() {
                    break; // exit condition
                }

                let mut split = current_line_trimmed.split('-');
                let range_start = split
                    .next()
                    .expect("Line is empty")
                    .parse::<usize>()
                    .expect("Invalid range start");
                let range_end = split
                    .next()
                    .expect("Invalid range")
                    .parse::<usize>()
                    .expect("Invalid range end");

                let range_end_merged = ranges
                    .get(&range_start)
                    .unwrap_or(&range_end)
                    .max(&range_end)
                    .clone();
                ranges.insert(range_start, range_end_merged);
                println!("{}-{}", range_start, range_end_merged);
            }
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                return ExitCode::FAILURE;
            }
        };
    }

    let mut first_part_solution = 0;

    // read ingredient IDs
    while let Some(line_res) = lines_iter.next() {
        match line_res {
            Ok(current_line) => {
                // parse number
                let current_num;
                match current_line.parse::<usize>() {
                    Ok(num) => current_num = num,
                    Err(_) => continue,
                }

                // iterate over ranges until range_start (key) is larger than current_num
                for (range_start, range_end) in ranges.range(..=current_num) {
                    if *range_end >= current_num {
                        println!("{} is fresh: {}-{}", current_num, range_start, range_end);
                        first_part_solution += 1;
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                return ExitCode::FAILURE;
            }
        }
    }

    println!("Number of fresh ingredients: {}", first_part_solution);

    ExitCode::SUCCESS
}
