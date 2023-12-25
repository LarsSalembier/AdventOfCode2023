use advent_of_code_2023::use_cases;
use advent_of_code_2023::adapters::file_io as file_io;

fn main() {
    let args = std::env::args().collect();
    let input = get_input_for_day(1).unwrap_or_else(|err| panic!("{}", err));
    let (day, part) = get_day_and_part_from_args(&args).unwrap_or_else(|err| panic!("{}", err));
    let solution = get_solution(day, part, &input).unwrap_or_else(|err| panic!("{}", err));
    println!("{}", solution);
}

fn get_input_for_day(day: i32) -> Result<Vec<String>, String> {
    let path = format!("inputs/day_{}.txt", day);
    let input = file_io::read_lines_as_vec(&path);

    if input.is_err() {
        return Err(format!("Could not read input file: {}", path));
    }

    Ok(input.unwrap())
}

fn get_solution(day: i32, part: i32, input: &Vec<String>) -> Result<String, String> {
    return match (day, part) {
        (1, 1) => Ok(use_cases::day_1::solve_part1(input).to_string()),
        _ => Err(format!("Day {} Part {} not implemented", day, part)),
    };
}

fn get_day_and_part_from_args(args: &Vec<String>) -> Result<(i32, i32), String> {
    if args.len() != 3 {
        return Err("Usage: cargo run <day> <part>".to_string());
    }

    let day = args[1].parse::<i32>();
    let part = args[2].parse::<i32>();

    if day.is_err() || part.is_err() {
        return Err("Day and part must be integers".to_string());
    }

    Ok((day.unwrap(), part.unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    mod mock_file_io {
        pub fn read_lines_as_vec(_: &str) -> Result<Vec<String>, std::io::Error> {
            Ok(vec!["Mocked line 1".to_string(), "Mocked line 2".to_string()])
        }
    }

    #[test]
    fn test_get_day_and_part_from_args_no_args() {
        let args: Vec<String> = vec![];

        let result = get_day_and_part_from_args(&args);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Usage: cargo run <day> <part>");
    }

    #[test]
    fn test_get_day_and_part_from_args_too_many_args() {
        let args = vec!["cargo".to_string(), "1".to_string(), "1".to_string(), "1".to_string()];

        let result = get_day_and_part_from_args(&args);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Usage: cargo run <day> <part>");
    }

    #[test]
    fn test_get_day_and_part_from_args_invalid_day() {
        let args = vec!["cargo".to_string(), "day".to_string(), "1".to_string()];

        let result = get_day_and_part_from_args(&args);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Day and part must be integers");
    }

    #[test]
    fn test_get_day_and_part_from_args_invalid_part() {
        let args = vec!["cargo".to_string(), "1".to_string(), "part".to_string()];

        let result = get_day_and_part_from_args(&args);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Day and part must be integers");
    }

    #[test]
    fn test_get_day_and_part_from_args_valid() {
        let args = vec!["cargo".to_string(), "1".to_string(), "1".to_string()];

        let result = get_day_and_part_from_args(&args);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (1, 1));
    }

    #[test]
    fn test_get_solution_invalid_day() {
        let result = get_solution(0, 1, &vec![]);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Day 0 Part 1 not implemented");
    }

    #[test]
    fn test_get_solution_invalid_part() {
        let result = get_solution(1, 0, &vec![]);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Day 1 Part 0 not implemented");
    }

    #[test]
    fn test_get_solution_valid() {
        let result = get_solution(1, 1, &vec![]);

        assert!(result.is_ok());
    }

    #[test]
    fn test_get_input_for_day_invalid_day() {
        let result = get_input_for_day(0);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Could not read input file: inputs/day_0.txt");
    }

    #[test]
    fn test_get_input_for_day_valid() {
        let result = get_input_for_day(1);

        assert!(result.is_ok());
    }
}
