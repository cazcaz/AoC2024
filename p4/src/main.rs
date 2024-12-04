use p4::*;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let grid = get_grid()?;
    let result1 = solution_one(&grid);
    let result2 = solution_two(&grid);
    println!("{result1}");
    println!("{result2}");
    Ok(())
}

fn solution_one(grid: &Vec<String>) -> i32 {
    search_xmas((&grid).to_vec())
}

fn solution_two<'a>(grid: &Vec<String>) -> i32 {
    search_x_mas((&grid).to_vec())
}

fn get_grid() -> Result<Vec<String>, &'static str> {
    let mut result: Vec<String> = Vec::new();
    if let Ok(file) = File::open("input.txt") {
        for line in io::BufReader::new(file).lines() {
            if let Ok(line) = line {
                result.push(line);
            }
        }
        Ok(result)
    } else {
        Err("Failed to open the file")
    }
}
