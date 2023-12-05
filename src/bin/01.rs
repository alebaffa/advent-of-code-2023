use regex::Regex;
use std::io::BufRead;
use std::str::FromStr;
advent_of_code::solution!(1);

pub fn part_one(_input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count: u32 = 0;
    for line in input.split('\n') {
        // Using expect because I want it to panic if there's a regex error
        let first_digit = first_decimal(&line.to_string()).expect("Regex error");
        let second_digit = first_decimal(&line.to_string().chars().rev().collect::<String>())
            .expect("Regex error");
        let two_digit_number = format!("{}{}", first_digit, second_digit);
        count += u32::from_str(&two_digit_number).expect("Parse error");
    }
    return Some(count);
}

fn first_decimal(input: &String) -> Result<String, regex::Error> {
    let digit_words_pattern = "one|two|three|four|five|six|seven|eight|nine";
    let reversed_digit_words_pattern = "eno|owt|eerht|ruof|evif|xis|neves|thgie|enin";
    let number_pattern = format!(
        r"({}|{}|\d)",
        digit_words_pattern, reversed_digit_words_pattern
    );
    let re = Regex::new(&number_pattern)?;

    let found = Ok(re
        .captures_iter(input)
        .filter_map(|caps| caps.get(0))
        .map(|matched| {
            match matched.as_str() {
                "one" | "eno" => "1",
                "two" | "owt" => "2",
                "three" | "eerht" => "3",
                "four" | "ruof" => "4",
                "five" | "evif" => "5",
                "six" | "xis" => "6",
                "seven" | "neves" => "7",
                "eight" | "thgie" => "8",
                "nine" | "enin" => "9",
                digit => digit,
            }
            .to_string()
        })
        .next()
        .unwrap_or_else(|| "0".to_string()));

    found
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }
}
