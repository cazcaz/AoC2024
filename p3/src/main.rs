use p3::*;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = get_string()?;
    let result_one = solution_one(input);
    println!("{}", result_one);
    Ok(())
}

fn solution_one(input: Vec<String>) -> i128 {
    let mut result = 0;
    for line in input {
        let results = parse_string(&line);
        result += mul_strings_to_result_one(results);
    }
    result
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
