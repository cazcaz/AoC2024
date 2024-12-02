use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

struct IncreasingTracker {
    set: bool,
    increasing: bool,
}

fn main() {
    let reports = get_reports();
    let result1 = solution_one(reports.clone());
    println!("Starting problem 2");
    let result2 = solution_two(reports);
    println!("{result1}");
    println!("{result2}");
}

fn solution_one<'a>(reports: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    for list in &reports {
        match validate_list_with_dampener(list.clone(), false) {
            Ok(_) => {
                result += 1;
            }
            Err(err) => {
                println!("Error validating list: {err}");
                dbg!(list);
            }
        }
    }
    result
}

fn solution_two<'a>(reports: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    for list in &reports {
        match validate_list_with_dampener(list.clone(), true) {
            Ok(_) => {
                result += 1;
            }
            Err(err) => {
                println!("Error validating list: {err}");
                dbg!(list);
            }
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

fn abs(a: i32) -> i32 {
    if a > 0 {
        a
    } else {
        -a
    }
}

fn validate_list_with_dampener(list: Vec<i32>, dampen: bool) -> Result<(), &'static str> {
    let mut last = list[0];
    let mut increasing = IncreasingTracker {
        set: false,
        increasing: false,
    };
    for (i, num) in list.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let diff = *num - last;
        if diff == 0 {
            if dampen {
                let mut new_list = list.clone();
                new_list.remove(i);
                dbg!(list);
                dbg!(&new_list);
                return validate_list_with_dampener(new_list, false);
                continue;
            }
            return Err("Duplicate number");
        }
        if !increasing.set {
            increasing.increasing = diff > 0;
            increasing.set = true;
        } else {
            if increasing.increasing != (diff > 0) {
                if dampen {
                    let mut new_list = list.clone();
                    new_list.remove(i);
                    dbg!(list);
                    dbg!(&new_list);
                    return validate_list_with_dampener(new_list, false);
                    continue;
                }
                return Err("Change of increase");
            }
        }
        if abs(diff) > 3 {
            if dampen {
                let mut new_list = list.clone();
                new_list.remove(i);
                dbg!(list);
                dbg!(&new_list);
                return validate_list_with_dampener(new_list, false);
                continue;
            }
            return Err("Too big of a difference");
        }
        last = *num;
    }

    Ok(())
}
