use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    process::ExitCode,
    vec,
};

use crate::{ch_num_conv::CharNumConverter, expr::Expression};

mod ch_num_conv;
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

fn safe_get_clone<T: Clone>(vec: &Vec<T>, index: usize, default_elem: T) -> T {
    match vec.get(index) {
        Some(val) => val.clone(),
        None => default_elem,
    }
}

fn safe_get_clone_iter<I>(iter: &mut I, index: usize, default_elem: I::Item) -> I::Item
where
    I: Iterator,
    I::Item: Clone,
{
    match iter.nth(index) {
        Some(val) => val.clone(),
        None => default_elem,
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
    let mut lines = vec![];
    process_file(&mut reader, |line| {
        // store line for second part
        lines.push(line.to_string());

        for (index, word) in line.split_ascii_whitespace().enumerate() {
            let expr = safe_get_mut(&mut expressions, index, Expression::new());

            // convert to number or print error and take it as 0
            let parsed_num = match word.parse::<u64>() {
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
    let first_part_solution = expressions.iter().fold(0, |acc, val| acc + val.calculate());

    //=======================================

    let mut expressions_2 = vec![];

    let longest_line_length = lines
        .iter()
        .map(|line| line.len())
        .max()
        .expect("No lines found!");
    let mut expr = Expression::new();
    for i in 0..longest_line_length {
        // get all characters in all columns and append to the number
        let mut col_ch_conv = CharNumConverter::new();
        for line in &lines {
            let ch = line.chars().nth(i).unwrap_or(' '); // if index is too large consider it a space character
            match ch {
                '+' | '*' => expr.operation = Some(ch),
                _ => col_ch_conv.push_digit(ch),
            }
        }

        if let Some(parsed_num) = col_ch_conv.num {
            // at least 1 digit found
            expr.nums.push(parsed_num);
        } else {
            // this is a space-only column, store expression and create new one
            expressions_2.push(expr);
            expr = Expression::new();
        }
    }
    expressions_2.push(expr); // push last one

    // do the calculations again
    let second_part_solution = expressions_2
        .iter()
        .fold(0, |acc, val| acc + val.calculate());

    eprintln!("{:?}", expressions_2);

    println!("Sum of all calculations: {}", first_part_solution);
    println!(
        "Sum of all vertical number calculations: {}",
        second_part_solution
    );

    ExitCode::SUCCESS
}
