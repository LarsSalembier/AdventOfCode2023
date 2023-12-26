//! Day 1

pub fn solve_part1(input: &[String]) -> String {
    let mut sum = 0;

    for line in input {
        let first_digit = line.chars().find(|c| c.is_digit(10));
        let last_digit = line.chars().rev().find(|c| c.is_digit(10));

        if let (Some(first), Some(last)) = (first_digit, last_digit) {
            if let Ok(num) = format!("{}{}", first, last).parse::<i32>() {
                sum += num;
            }
        } else {
            return format!("Invalid input found: line \"{}\" should contain at least one digit", line);
        }
    }

    sum.to_string()
}

pub fn solve_part2(input: &[String]) -> String {
    let mut sum = 0;
    let digit_map = create_digit_map();

    for line in input {
        let first_digit = find_first_digit(&line, &digit_map);
        let last_digit = find_last_digit(&line, &digit_map);

        match (first_digit, last_digit) {
            (Some(first), Some(last)) => {
                if let Ok(num) = format!("{}{}", first, last).parse::<i32>() {
                    sum += num;
                }
            }
            _ => return format!("Invalid input found: line \"{}\" should contain at least one digit", line),
        }
    }

    sum.to_string()
}

fn create_digit_map() -> std::collections::HashMap<&'static str, char> {
    vec![
        ("one", '1'), ("two", '2'), ("three", '3'), ("four", '4'),
        ("five", '5'), ("six", '6'), ("seven", '7'), ("eight", '8'), ("nine", '9'),
    ].into_iter().collect()
}

fn find_first_digit(line: &str, digit_map: &std::collections::HashMap<&str, char>) -> Option<char> {
    let mut current_word = String::new();

    for ch in line.chars() {
        if ch.is_digit(10) {
            return Some(ch);
        } else if ch.is_alphabetic() {
            current_word.push(ch);
            // Check for any digit word that matches at the end of the current word
            for (word, &digit) in digit_map {
                if current_word.ends_with(word) {
                    return Some(digit);
                }
            }
        } else {
            current_word.clear();
        }
    }

    None
}

fn find_last_digit(line: &str, digit_map: &std::collections::HashMap<&str, char>) -> Option<char> {
    let reversed_line = line.chars().rev().collect::<String>();
    let mut current_word = String::new();

    for ch in reversed_line.chars() {
        if ch.is_digit(10) {
            return Some(ch);
        } else if ch.is_alphabetic() {
            current_word.insert(0, ch);
            // Check for any digit word that matches at the start of the current word
            for (word, &digit) in digit_map {
                if current_word.starts_with(word) {
                    return Some(digit);
                }
            }
        } else {
            current_word.clear();
        }
    }

    None
}