use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    process::ExitCode,
};

use crate::matrix::Matrix;

mod matrix;

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

    // open file
    let reader;
    match File::open(filename.clone()) {
        Ok(file) => reader = BufReader::new(file),
        Err(e) => {
            eprintln!("Error opening {}: {}", filename, e);
            return ExitCode::FAILURE;
        }
    }

    let mut line_iter = reader.lines();

    // read first line, calculate length
    let first_line = line_iter
        .next()
        .expect("Error reading file")
        .expect("Your file must have at least one line!");
    let first_line_len = first_line.chars().count();

    // create matrix and parse first line
    let mut grid = Matrix::new(first_line_len);
    match grid.push_line(first_line.as_str()) {
        true => println!("{}", grid.get_last_line('.', '@').unwrap()),
        false => println!("<Empty line>"),
    }

    // read and parse the rest of the lines
    for line in line_iter.map(|res| res.expect("Error reading file")) {
        match grid.push_line(line.as_str()) {
            true => println!("{}", grid.get_last_line('.', '@').unwrap()),
            false => println!("<Empty line>"),
        }
    }

    ExitCode::SUCCESS
}
