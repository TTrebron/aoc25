use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    process::ExitCode,
};

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

fn safe_get_mut_iter<I>(iter: &mut I, index: usize, default_elem: I::Item) -> I::Item
where
    I: Iterator,
    I::Item: Clone,
{
    match iter.nth(index) {
        Some(val) => val.clone(),
        None => default_elem,
    }
}

fn init_state(state: &mut Vec<u64>, line_len: usize) {
    state.resize(line_len, 0);
}

fn update_state(state: &mut Vec<u64>, line: &str) -> u32 {
    let mut splits = 0;
    for i in 1..state.len().checked_sub(1).unwrap_or(0) {
        match line.chars().nth(i) {
            Some('S') => state[i] = 1,
            Some('^') => {
                if state[i] > 0 {
                    splits += 1;
                    state[i - 1] += state[i];
                    state[i + 1] += state[i];
                    state[i] = 0;
                }
            }
            _ => (),
        }
    }

    splits
}

fn main() -> ExitCode {
    let Some(filename) = env::args().nth(1) else {
        eprintln!(
            "Usage: {} <filename>",
            env::args().nth(0).unwrap_or_default()
        );
        return ExitCode::FAILURE;
    };

    let mut reader;
    match File::open(filename.as_str()) {
        Ok(file) => reader = BufReader::new(file),
        Err(e) => {
            eprintln!("Cannot open file {}: {}", filename.as_str(), e);
            return ExitCode::FAILURE;
        }
    }

    let mut first_part_solution = 0;

    let mut state = vec![];
    process_file(&mut reader, |line| {
        if state.len() == 0 {
            init_state(&mut state, line.len());
        }
        first_part_solution += update_state(&mut state, line);
    });

    let mut second_part_solution = 0;
    for timeline_count in state {
        second_part_solution += timeline_count;
    }

    println!(
        "The number of beam splits in the diagram: {}",
        first_part_solution
    );

    println!(
        "The number of timelines in the diagram: {}",
        second_part_solution
    );

    ExitCode::SUCCESS
}
