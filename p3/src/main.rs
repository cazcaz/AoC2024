use p3::*;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = get_string()?;
    let result_one = solution_one(input.clone());
    let result_two = solution_two(input);
    println!("{}", result_one);
    println!("{}", result_two);
    Ok(())
}

fn solution_one(input: Vec<String>) -> i128 {
    // Combine test input into one string
    let combined_string: &str = &input.join("");
    let results = parse_string(&combined_string);
    mul_strings_to_result(results)
}

fn solution_two(input: Vec<String>) -> i128 {
    // Combine test input into one string so that the current do() or don't() state is remembered between lines
    let combined_string: &str = &input.join("");
    let results_with_commands = parse_string_with_commands(&combined_string);
    let results = process_commands(results_with_commands);
    mul_strings_to_result(results)
}

fn get_string() -> Result<Vec<String>, &'static str> {
    let mut result: Vec<String> = Vec::new();
    if let Ok(file) = File::open("input.txt") {
        for line in io::BufReader::new(file).lines() {
            if let Ok(line) = line {
                result.push(String::from(line));
            }
        }
    }
    Ok(result)
}
