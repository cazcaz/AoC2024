pub fn solution_one(input: &String) -> i128 {
    let mut result = 0;
    for line in input.lines() {
        let problem = parse_input(line);
        if resolve_computation(&problem.1, problem.0) {
            result += problem.0;
        }
    }
    result
}
pub fn solution_two(input: &String) -> i128 {
    let mut result = 0;
    for line in input.lines() {
        let problem = parse_input(line);
        if resolve_computation_concatenate(&problem.1, problem.0) {
            result += problem.0;
        }
    }
    result
}

fn resolve_computation(operands: &Vec<i128>, target: i128) -> bool {
    if target < 0 {
        return false;
    }
    if operands.len() <= 0 {
        panic!("Empty operands list");
    }
    if operands.len() == 1 {
        if operands[0] == target {
            return true;
        } else {
            return false;
        }
    }
    let mut op_clone = operands.clone();
    if let Some(last) = op_clone.pop() {
        if resolve_computation(&op_clone, target - last) {
            return true;
        }
        if target % last == 0 && resolve_computation(&op_clone, target / last) {
            return true;
        }
    } else {
        panic!("Vector should not be empty");
    }
    false
}

fn resolve_computation_concatenate(operands: &Vec<i128>, target: i128) -> bool {
    if target < 0 {
        return false;
    }
    if operands.len() <= 0 {
        panic!("Empty operands list");
    }
    if operands.len() == 1 {
        if operands[0] == target {
            return true;
        } else {
            return false;
        }
    }
    let mut op_clone = operands.clone();
    if let Some(last) = op_clone.pop() {
        if resolve_computation_concatenate(&op_clone, target - last) {
            return true;
        }
        if target % last == 0 && resolve_computation_concatenate(&op_clone, target / last) {
            return true;
        }

        if let Ok(val) = concatenated_target(&last, &target) {
            if resolve_computation_concatenate(&op_clone, val) {
                return true;
            }
        }
    } else {
        panic!("Vector should not be empty");
    }
    false
}

fn parse_input(input: &str) -> (i128, Vec<i128>) {
    let parts: Vec<&str> = input.split(':').collect();
    let first_number = parts[0].trim().parse::<i128>().expect("Invalid number");
    let numbers: Vec<i128> = parts[1]
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();
    (first_number, numbers)
}

// This will apply the opposite of the concatenation to the RHS
fn concatenated_target<'a>(dividend: &'a i128, target: &'a i128) -> Result<i128, &'a str> {
    let mut new_target = target - dividend;
    if new_target % 10 != 0 {
        return Err("Target not valid");
    }
    if new_target < 0 {
        return Err("Target too small");
    }
    let digits = dividend.ilog(10) + 1;
    for _ in 0..digits {
        new_target /= 10;
    }
    Ok(new_target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_computation() {
        let input: Vec<(Vec<i128>, i128)> = vec![
            (vec![10, 19], 190),
            (vec![81, 40, 27], 3267),
            (vec![17, 5], 83),
            (vec![15, 6], 156),
            (vec![6, 8, 6, 15], 7290),
            (vec![16, 10, 13], 161011),
            (vec![17, 8, 14], 192),
            (vec![9, 7, 18, 13], 21037),
            (vec![11, 6, 16, 20], 292),
        ];
        let expected: i128 = 3749;
        let mut result = 0;
        for args in input {
            if resolve_computation(&args.0, args.1) {
                result += args.1;
            }
        }
        assert_eq!(result, expected);
    }

    #[test]
    fn test_resolve_computation_concatenate() {
        let input: Vec<(Vec<i128>, i128)> = vec![
            (vec![10, 19], 190),
            (vec![81, 40, 27], 3267),
            (vec![17, 5], 83),
            (vec![15, 6], 156),
            (vec![6, 8, 6, 15], 7290),
            (vec![16, 10, 13], 161011),
            (vec![17, 8, 14], 192),
            (vec![9, 7, 18, 13], 21037),
            (vec![11, 6, 16, 20], 292),
        ];
        let expected: i128 = 11387;
        let mut result = 0;
        for args in input {
            if resolve_computation_concatenate(&args.0, args.1) {
                result += args.1;
            }
        }
        assert_eq!(result, expected);
    }

    #[test]
    fn test_concatenated_target() {
        let input = vec![(6, 486), (1, 1), (187192, 25187192), (123, 456), (123, 23)];
        let expected = vec![
            Ok(48),
            Ok(0),
            Ok(25),
            Err("Target not valid"),
            Err("Target too small"),
        ];
        for (i, input_val) in input.iter().enumerate() {
            assert_eq!(concatenated_target(&input_val.0, &input_val.1), expected[i]);
        }
    }
}
