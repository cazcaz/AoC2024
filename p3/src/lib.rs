use regex::Regex;

pub fn parse_string<'a>(input: &'a str) -> Vec<&'a str> {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let mut results = vec![];
    let captures = re.captures_iter(input);
    for cap in captures {
        results.push(cap.get(0).unwrap().as_str());
    }
    results
}

pub fn parse_string_with_commands<'a>(input: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    results
}

fn process_commands(mul_strings: Vec<&str>) -> Vec<&str> {
    let mut results = vec![];
    results
}

pub fn parse_mul(mul_string: &str) -> (i128, i128) {
    let sliced = &mul_string[4..mul_string.len() - 1];
    let mut split = sliced.split(",");
    let x = split.next().unwrap().parse::<i128>().unwrap();
    let y = split.next().unwrap().parse::<i128>().unwrap();
    (x, y)
}

pub fn mul_strings_to_result_one(mul_strings: Vec<&str>) -> i128 {
    let mut result = 0;
    for mul_string in mul_strings {
        let (x, y) = parse_mul(mul_string);
        result += x * y;
    }
    result
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parse_mul_single_digits() {
        let input = "mul(1,2)";
        let expected = (1, 2);
        assert_eq!(parse_mul(input), expected);
    }

    #[test]
    fn test_parse_mul_different_lengths() {
        let input = "mul(1,23)";
        let expected = (1, 23);
        assert_eq!(parse_mul(input), expected);

        let input2 = "mul(123,4)";
        let expected2 = (123, 4);
        assert_eq!(parse_mul(input2), expected2);

        let input3 = "mul(1234,567)";
        let expected3 = (1234, 567);
        assert_eq!(parse_mul(input3), expected3);
    }

    #[test]
    fn test_parse_mul_negative_numbers() {
        let input = "mul(-1,2)";
        let expected = (-1, 2);
        assert_eq!(parse_mul(input), expected);

        let input2 = "mul(1,-2)";
        let expected2 = (1, -2);
        assert_eq!(parse_mul(input2), expected2);

        let input3 = "mul(-1,-2)";
        let expected3 = (-1, -2);
        assert_eq!(parse_mul(input3), expected3);
    }

    #[test]
    fn test_parse_string() {
        let input = "Hello, world!";
        let expected: Vec<&str> = vec![];
        assert_eq!(parse_string(input), expected);

        let input2 = "Hello, world! mul(1,2)";
        let expected2 = vec!["mul(1,2)"];
        assert_eq!(parse_string(input2), expected2);

        let input3 = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let expected3 = vec!["mul(2,4)", "mul(5,5)", "mul(11,8)", "mul(8,5)"];
        assert_eq!(parse_string(input3), expected3);
    }

    #[test]
    fn test_mul_strings_to_result_one() {
        let input = vec!["mul(1,2)", "mul(3,4)", "mul(5,6)"];
        let expected = 1 * 2 + 3 * 4 + 5 * 6;
        assert_eq!(mul_strings_to_result_one(input), expected);
    }

    #[test]
    fn test_mul_strings_test_input() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let expected = 161;
        assert_eq!(mul_strings_to_result_one(parse_string(input)), expected);
    }

    #[test]
    fn test_parse_string_part_two() {
        let input = "mul(1,2)mul(3,4)don't()mul(5,6)";
        let expected = vec!["mul(1,2)", "mul(3,4)", "don't()", "mul(5,6)"];
        assert_eq!(parse_string_with_commands(input), expected);

        let input2 = "don't()mul(1,2)mul(3,4)mul(5,6)mul(7,8)";
        let expected2: Vec<&str> = vec!["don't()", "mul(1,2)", "mul(3,4)", "mul(5,6)", "mul(7,8)"];
        assert_eq!(parse_string_with_commands(input2), expected2);

        let input3 = "mul(1,2)don't()mul(3,4)mul(5,6)do()mul(7,8)don't()";
        let expected3 = vec![
            "mul(1,2)", "don't()", "mul(3,4)", "mul(5,6)", "do()", "mul(7,8)", "don't()",
        ];
        assert_eq!(parse_string_with_commands(input3), expected3);

        let input4 = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let expected4 = vec![
            "mul(2,4)",
            "don't()",
            "mul(5,5)",
            "mul(11,8)",
            "do()",
            "mul(8,5)",
        ];
        assert_eq!(parse_string_with_commands(input4), expected4);
    }

    #[test]
    fn test_mul_strings_to_result_two() {
        let input = vec!["mul(1,2)", "mul(3,4)", "don't()", "mul(5,6)"];
        let expected = vec!["mul(1,2)", "mul(3,4)"];
        assert_eq!(process_commands(input), expected);

        let input2 = vec!["don't()", "mul(1,2)", "mul(3,4)", "mul(5,6)", "mul(7,8)"];
        let expected2: Vec<&str> = vec![];
        assert_eq!(process_commands(input2), expected2);

        let input3 = vec![
            "mul(1,2)", "don't()", "mul(3,4)", "mul(5,6)", "do()", "mul(7,8)", "don't()",
        ];
        let expected3 = vec!["mul(1,2)", "mul(7,8)"];
        assert_eq!(process_commands(input3), expected3);
    }
}
