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

    // read and parse ranges
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

    // iterate over ranges, keep previous. is previous range_end >= current range_start -> previous range_end = current range_start - 1
    let mut prev_range: Option<(usize, usize)> = None;
    for (range_start, range_end) in ranges.clone() {
        let Some((prev_range_start, prev_range_end)) = prev_range else {
            prev_range = Some((range_start, range_end));
            continue;
        };

        // if current range starts in previous range:
        let mut prev_set = true;
        if prev_range_end >= range_start {
            if range_end <= prev_range_end {
                // the range is fully included in the previous range - remove and don't update previous range
                println!(
                    "range {}-{} removed because its in range {}-{}",
                    range_start, range_end, prev_range_start, prev_range_end
                );
                ranges.remove(&range_start);
                prev_set = false;
            } else {
                // simply shrink the previous range
                ranges.insert(prev_range_start, range_start - 1);
            }
        }

        if prev_set {
            prev_range = Some((range_start, range_end));
        }
    }

    // calculate number of all IDs
    let mut second_part_solution = 0;
    for (range_start, range_end) in ranges.clone() {
        second_part_solution += range_end - range_start + 1;
    }

    println!("Number of fresh ingredients: {}", first_part_solution);
    println!("Number of all fresh ingredients: {}", second_part_solution);

    ExitCode::SUCCESS
}
