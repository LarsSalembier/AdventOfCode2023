//! Advent of Code 2023 Solution Runner
//!
//! This program is designed to run solutions for the Advent of Code 2023 challenges.
//! It requires two command-line arguments: the day and the part of the challenge to solve.
//!
//! # Usage
//! ```bash
//! cargo run <day> <part>
//! ```
//! `<day>` and `<part>` should be replaced with the specific day and part number of the challenge.
//!
//! # Examples
//! ```bash
//! cargo run 1 1
//! ```
//! This will run the solution for Day 1, Part 1.
//!
//! # Notes
//! When run, this program will look for a file named `inputs/day_<day>.txt` in the current directory,
//! where `<day>` is the day number specified in the command-line arguments. If the file is not found,
//! the program will exit with an error. If the file is found, but cannot be read, the program will
//! exit with an error.
//!
//! The program will also fail to run if command-line arguments are not provided, or if the arguments
//! are invalid. The first argument must be a positive integer, and the second argument must be either
//! 1 or 2.
//!
//! The program will also look for a file named `inputs/day_<day>.txt` in the `target/debug` directory.
//! This is because the `build.rs` script will copy all files from the `inputs` directory to the
//! `target/debug/inputs` directory. This is done so that the program can be run from the command line
//! without having to specify the path to the input file.
//!
//! # License
//! This program is licensed under the MIT license. See the [LICENSE](LICENSE) file for more information.
//!
//! # Author
//! This program was written by [Lars Salembier](https://github.com/LarsSalembier).

use std;
use std::process;

use advent_of_code_2023::adapters::file_io as file_io;
use advent_of_code_2023::domain::error_handling::ProgramError;
use advent_of_code_2023::domain::models::day::Day;
use advent_of_code_2023::domain::models::part::Part;
use advent_of_code_2023::use_cases;

/// The main entry point for the Advent of Code 2023 solution runner.
fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let (day, part) = get_day_and_part_from_args(&args).unwrap_or_else(handle_error_and_exit);
    let input = get_input_for_day(&day).unwrap_or_else(|err| handle_error_and_exit(ProgramError::IoError(err)));
    let solution = get_solution(&day, &part, &input).unwrap_or_else(handle_error_and_exit);
    println!("{}", solution);
}

/// Handles errors by printing them and then exiting the program.
///
/// # Arguments
/// * `error` - The error to handle.
///
/// # Type Parameters
/// * `T` - The type of the value to return. This function will never return a value, but this
///        parameter is required to satisfy the compiler.
///
/// # Returns
/// This function will never return a value. It will always exit the program.
fn handle_error_and_exit<T>(error: ProgramError) -> T {
    eprintln!("Error: {}", error);
    process::exit(1);
}

/// Retrieves the input for a given day of the Advent of Code challenge. The input is expected to be
/// in a file named `inputs/day_<day>.txt` in the current directory.
///
/// # Arguments
/// * `day` - The day of the challenge.
///
/// # Returns
/// A `Result` which is `Ok` containing the input data as a vector of strings if successful,
/// or an `Err` containing an `std::io::Error` if an error occurs.
fn get_input_for_day(day: &Day) -> Result<Vec<String>, std::io::Error> {
    let path = format!("inputs/day_{}.txt", day.value());
    file_io::read_lines_as_vec(&path)
}

/// Fetches the solution for a given day and part of the Advent of Code challenge.
///
/// # Arguments
/// * `day` - The day of the challenge.
/// * `part` - The part of the challenge.
/// * `input` - The input data for the challenge.
///
/// # Returns
/// A `Result` which is `Ok` containing the solution as a string if successful, or an `Err` containing
/// a `ProgramError::NotYetImplemented` if the solution has not yet been implemented.
fn get_solution(day: &Day, part: &Part, input: &[String]) -> Result<String, ProgramError> {
    let day = day.value();
    let part = part.value();

    match (day, part) {
        (1, 1) => Ok(use_cases::day_1::solve_part1(input).to_string()),
        _ => Err(ProgramError::NotYetImplemented(format!("Day {} Part {} not implemented", day, part))),
    }
}

/// Parses the command-line arguments and returns the day and part of the challenge to solve.
/// If the arguments are invalid, an error is returned.
///
/// The first argument must be a positive integer, and the second argument must be either 1 or 2.
///
/// # Arguments
/// * `args` - The command-line arguments.
///
/// # Returns
/// A `Result` which is `Ok` containing the day and part of the challenge to solve if successful,
/// or an `Err` containing a `ProgramError::InputError` if the arguments are invalid.
fn get_day_and_part_from_args(args: &[String]) -> Result<(Day, Part), ProgramError> {
    if args.len() != 3 {
        return Err(ProgramError::InputError("Invalid arguments! Usage: cargo run <day> <part>".to_string()));
    }
    let day = args[1].parse::<i32>().map_err(|_| ProgramError::InputError("Day must be a positive integer".to_string()))?;
    let part = args[2].parse::<i32>().map_err(|_| ProgramError::InputError("Part must be either 1 or 2".to_string()))?;

    let day = Day::new(day).map_err(|_| ProgramError::InputError("Day must be a positive integer".to_string()))?;
    let part = Part::new(part).map_err(|_| ProgramError::InputError("Part must be either 1 or 2".to_string()))?;

    Ok((day, part))
}
