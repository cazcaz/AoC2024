use p8::*;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = get_input()?;
    let result1 = solution_one(&input);
    let result2 = solution_two(&input);
    println!("{result1}");
    println!("{result2}");
    Ok(())
}

fn get_input() -> Result<String, &'static str> {
    let mut result = String::new();
    if let Ok(mut file) = File::open("input.txt") {
        if let Ok(_string) = file.read_to_string(&mut result) {
            return Ok(result);
        } else {
            return Err("Failed to read the string");
        }
    }
    Err("Failed to open the file")
}
