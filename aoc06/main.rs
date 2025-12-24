use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    process::ExitCode,
};

use crate::expr::Expression;

mod expr;

fn process_file<F>(reader: &mut BufReader<File>, mut process_line: F)
where
    F: FnMut(&str),
{
    // run process_line for each line in reader
    for line_option in reader.lines() {
        match line_option {
            Ok(line) => process_line(line.as_str()),
            Err(e) => {
                eprintln!("Failed to read line: {}", e);
            }
        }
    }
}

fn safe_get_mut<T: Clone>(vec: &mut Vec<T>, index: usize, default_elem: T) -> &mut T {
    if index >= vec.len() {
        vec.resize(index + 1, default_elem.clone());
        vec.insert(index, default_elem.clone());
    }
    vec.get_mut(index).unwrap()
}

fn main() -> ExitCode {
    // get filename
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

    // open file
    let mut reader = match File::open(filename.as_str()) {
        Ok(file) => BufReader::new(file),
        Err(e) => {
            eprintln!("Error opening {}: {}", filename, e);
            return ExitCode::FAILURE;
        }
    };

    // create expression container and process file line by line
    let mut expressions = vec![];
    process_file(&mut reader, |line| {
        for (index, word) in line.split_ascii_whitespace().enumerate() {
            let expr = safe_get_mut(&mut expressions, index, Expression::new());

            // convert to number or print error and take it as 0
            let parsed_num = match word.parse::<i64>() {
                Ok(num) => Some(num),
                Err(_) => {
                    // cannot convert, try parsing operator
                    match word {
                        "+" => expr.operation = Some('+'),
                        "*" => expr.operation = Some('*'),
                        _ => eprintln!(
                            "Cannot parse number {} at column {} - ignoring",
                            word, index
                        ),
                    }
                    None
                }
            };

            if parsed_num.is_some() {
                // if a number could be parsed, store it for calculation later
                expr.nums.push(parsed_num.unwrap());
            }
            eprintln!("{:?}", expr.nums);
        }
    });

    // do the calculations
    let mut first_part_solution = 0;
    for expr in expressions {
        first_part_solution += match expr.operation {
            Some('+') => expr.nums.iter().sum::<i64>(),
            Some('*') => expr.nums.iter().product::<i64>(),
            Some(_) | None => 0,
        };
    }

    println!("Sum of all calculations: {}", first_part_solution);

    ExitCode::SUCCESS
}
