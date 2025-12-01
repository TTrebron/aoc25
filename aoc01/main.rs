use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    process::ExitCode,
};

use crate::dial::Dial;

mod dial;

fn open_file(filename: &str) -> Result<BufReader<File>, io::Error> {
    match File::open(filename) {
        Ok(file) => return Ok(BufReader::new(file)),
        Err(e) => return Err(e),
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <inputfile>", args[0]);
        return ExitCode::FAILURE;
    }

    let filename = args[1].as_str();
    // open file, exit if failed
    let reader;
    match open_file(&filename) {
        Ok(value) => reader = value,
        Err(_) => {
            eprintln!("Error reading {}", filename);
            return ExitCode::FAILURE;
        }
    }

    let mut solution_part_one = 0;
    let mut solution_part_two = 0;
    let mut dial = Dial::new(50);
    for read_next_line in reader.lines() {
        match read_next_line {
            Ok(line) => {
                // print line
                println!("{}", line);
                // parse line: returns Some(-n) or Some(n) or None
                match Dial::parse_line(&line) {
                    Some((instruction, hundreds)) => {
                        // rotate_dial(instruction) -> returns true if result is 0
                        let (current_state, hundred_overflow) = dial.rotate(instruction);
                        if current_state == 0 {
                            // if return is 0: solution += 1;
                            solution_part_one += 1;
                        }
                        // part two
                        solution_part_two += hundreds;
                        if hundred_overflow {
                            solution_part_two += 1;
                        }
                    }
                    None => eprintln!("Error parsing line"),
                }
            }
            Err(_) => {
                eprintln!("Error reading next line");
            }
        }
    }

    println!("Number of 0 states: {}", solution_part_one);
    println!("Number of 0 states during rotations: {}", solution_part_two);

    ExitCode::SUCCESS
}
