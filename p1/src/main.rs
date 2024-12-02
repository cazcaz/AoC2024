use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let (left, right) = get_lists();
    let result1 = solution_one(left.clone(), right.clone());
    let result2 = solution_two(left, right);
    println!("{result1}");
    println!("{result2}");
}

fn solution_one<'a>(mut left: Vec<i128>, mut right: Vec<i128>) -> i128 {
    let mut result = 0;

    left.sort();
    right.sort();

    for (i, left_item) in left.iter().enumerate() {
        let dist = left_item - right[i];
        if dist < 0 {
            result -= dist;
        } else {
            result += dist;
        }
    }
    result
}

fn solution_two<'a>( left: Vec<i128>, right: Vec<i128>) -> i128 {
    let mut result = 0;

    let mut right_map = HashMap::new();

    for num in right {
        let count = right_map.entry(num).or_insert(0);
        *count += 1;
    }

    for num in left {
        if let Some(count) = right_map.get(&num) {
            result += num * count;
        }
    }

    result
}

fn get_lists() -> (Vec<i128>, Vec<i128>) {
    let mut left_list: Vec<i128> = Vec::new();
    let mut right_list: Vec<i128> = Vec::new();
    if let Ok(file) = File::open("input.txt") {
        for line in io::BufReader::new(file).lines() {
            if let Ok(line) = line {
                let num_strings: Vec<&str> = line.split("   ").collect();
                let mut first = true;
                for num_string in num_strings {
                    match num_string.parse::<i128>() {
                        Ok(num) => {
                            if first {
                                left_list.push(num)
                            } else {
                                right_list.push(num)
                            }
                        }
                        Err(_) => (),
                    }
                    first = false;
                }
            }
        }
    }
    assert_eq!(left_list.len(), right_list.len());
    (left_list, right_list)
}
