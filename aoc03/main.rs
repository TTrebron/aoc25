use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    process::ExitCode,
};

fn get_largest_char_index<I: Iterator<Item = char>>(line: I, from: usize) -> Option<(usize, char)> {
    let mut line_enum = line
        .enumerate() // [(0, &val), (1, &val), (2, &val), ...]
        .skip(from); // skip n elements

    let first_elem = match line_enum.next() {
        Some(elem) => elem,
        None => return None
    }; // strip the first element

    return Some(line_enum.fold(first_elem, |(max_key, max_val), (key, val)| {
        if val > max_val {
            (key, val) // only store the next index if the value is larger (and not when they are equal)
        } else {
            (max_key, max_val)
        }
    })); // get max, the first one if multiple
    //.max_by_key(|&(_, v)| v) // computes max value, returns (index, &val), but always returns the last occurrence of the max value
}

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

    let file;
    let reader;
    match File::open(filename.as_str()) {
        Ok(obj) => {
            file = obj;
            reader = BufReader::new(file);
        }
        Err(err) => {
            eprintln!("Error reading {}: {}", filename.as_str(), err);
            return ExitCode::FAILURE;
        }
    }

    let mut first_part_solution = 0;
    for read_next_line in reader.lines() {
        match read_next_line {
            Ok(line) => {
                let count_minus_one = match line.chars().count().checked_sub(1) {
                    None => continue, // line is empty
                    Some(0) => continue, // line consists of one char
                    Some(key) => key,
                };

                let (largest_char_index, largest_char) = match get_largest_char_index(line.chars().take(count_minus_one), 0) {
                    Some((key, val)) => (key, val),
                    None => continue
                };
                let (largest_char_after_index, largest_char_after) = match get_largest_char_index(line.chars(), largest_char_index + 1) {
                    Some((key, val)) => (key, val),
                    None => continue
                };

                println!("{}, {}", largest_char_index, largest_char_after_index);

                let largest_combined = largest_char
                    .to_digit(10)
                    .expect("Every character must be a digit")
                    * 10
                    + largest_char_after
                        .to_digit(10)
                        .expect("Every character must be a digit");
                println!("{} largest: {}", line, largest_combined);
                first_part_solution += largest_combined;
            }
            Err(err) => {
                eprintln!("Error reading {}: {}", filename.as_str(), err);
                return ExitCode::FAILURE;
            }
        }
    }

    println!(
        "Sum of largest two-digit numbers with consecutive digits: {}",
        first_part_solution
    );

    return ExitCode::SUCCESS;
}
