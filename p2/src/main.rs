use p2::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let reports = get_reports();
    let result1 = solution_one(reports.clone());
    let result2 = solution_two(reports);
    println!("{result1}");
    println!("{result2}");
}

fn solution_one<'a>(reports: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    for list in &reports {
        if validate_list(list) {
            result += 1;
        }
    }
    result
}

fn solution_two<'a>(reports: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    for list in &reports {
        if validate_list_with_damp(list) {
            result += 1;
        }
    }
    result
}

fn get_reports() -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    if let Ok(file) = File::open("input.txt") {
        for line in io::BufReader::new(file).lines() {
            if let Ok(line) = line {
                let mut line_list: Vec<i32> = Vec::new();
                let num_strings: Vec<&str> = line.split(" ").collect();
                for num_string in num_strings {
                    match num_string.parse::<i32>() {
                        Ok(num) => {
                            line_list.push(num);
                        }
                        Err(_) => (),
                    }
                }
                reports.push(line_list);
            }
        }
    }
    reports
}
