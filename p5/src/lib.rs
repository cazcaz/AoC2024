// use test_helpers;
use std::cmp::Ordering;
use std::collections::HashMap;

struct Ruleset {
    rules: HashMap<i32, Vec<i32>>,
}

impl Ruleset {
    fn new_from_map(map: HashMap<i32, Vec<i32>>) -> Self {
        Self { rules: map.clone() }
    }

    fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    fn insert_rule(&mut self, smaller: i32, larger: i32) {
        self.rules
            .entry(smaller)
            .or_insert_with(Vec::new)
            .push(larger);
    }

    fn insert_rule_string(&mut self, rule: String) {
        let rules: Vec<i32> = rule
            .split("|")
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        if rules.len() != 2 {
            panic!("Did not have 2 rules after parsing the rule string");
        }
        self.insert_rule(rules[0], rules[1]);
    }

    // Returns true is first < second according to the rules
    fn check_order(&self, first: &i32, second: &i32) -> bool {
        if let Some(rule_list) = self.rules.get(first) {
            rule_list.contains(second)
        } else {
            false
        }
    }

    fn custom_order(&self, first: &i32, second: &i32) -> Ordering {
        if self.check_order(first, second) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

fn split_input(input: &String) -> (Vec<String>, Vec<String>) {
    // Remove spaces (but not newlines)
    let stripped: String = input.chars().filter(|c| !(*c == ' ')).collect();

    // Split into two blocks by the double new line
    let blocks: Vec<String> = stripped.split("\n\n").map(String::from).collect();

    if blocks.len() != 2 {
        panic!("Input was split into more or less than 2 strings");
    }

    let rules: Vec<String> = blocks[0].lines().map(String::from).collect();
    let pages: Vec<String> = blocks[1].lines().map(String::from).collect();

    (rules, pages)
}

fn check_list(list: &Vec<i32>, rule_set: &Ruleset) -> bool {
    for i in 0..list.len() - 1 {
        let smaller = list[i] as i32;
        for j in i + 1..list.len() {
            let larger = list[j] as i32;
            if !rule_set.check_order(&smaller, &larger) {
                return false;
            }
        }
    }
    true
}

pub fn solution_one(input: &String) -> i32 {
    let mut result = 0;
    let split_inputs = split_input(input);
    let mut rules = Ruleset::new();
    for str_rules in split_inputs.0 {
        rules.insert_rule_string(str_rules);
    }
    for line in split_inputs.1 {
        let page_vec: Vec<i32> = line
            .split(",")
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        if check_list(&page_vec, &rules) {
            let middle_id = page_vec.len() / 2;
            result += page_vec[middle_id];
        }
    }
    result
}

pub fn solution_two(input: &String) -> i32 {
    let mut result = 0;
    let split_inputs = split_input(input);
    let mut rules = Ruleset::new();
    for str_rules in split_inputs.0 {
        rules.insert_rule_string(str_rules);
    }

    let mut pages: Vec<Vec<i32>> = vec![];
    for line in split_inputs.1 {
        let page_vec: Vec<i32> = line
            .split(",")
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        pages.push(page_vec.clone());
    }
    let incorrect_pages: Vec<Vec<i32>> = pages
        .into_iter()
        .filter(|page_list| !check_list(&page_list, &rules))
        .collect();

    let sorted_pages: Vec<Vec<i32>> = incorrect_pages
        .into_iter()
        .map(|list| correct_list(&list, &rules))
        .collect();

    for list in sorted_pages {
        let middle_id = list.len() / 2;
        result += list[middle_id];
    }

    result
}

fn correct_list(list: &Vec<i32>, rule_set: &Ruleset) -> Vec<i32> {
    let mut sorted: Vec<i32> = list.clone();
    sorted.sort_by(|first, second| rule_set.custom_order(first, second));
    sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_input() {
        let input = String::from(
            "47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13
        
        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47",
        );

        let expected: (Vec<String>, Vec<String>) = (
            vec![
                "47|53", "97|13", "97|61", "97|47", "75|29", "61|13", "75|53", "29|13", "97|29",
                "53|29", "61|53", "97|53", "61|29", "47|13", "75|47", "97|75", "47|61", "75|61",
                "47|29", "75|13", "53|13",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            vec![
                "75,47,61,53,29",
                "97,61,53,29,13",
                "75,29,13",
                "75,97,47,61,53",
                "61,13,29",
                "97,13,75,29,47",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
        );

        assert_eq!(split_input(&input), expected);
    }

    #[test]
    fn test_ruleset_creation() {
        let input: Vec<String> = vec![
            "47|53", "97|13", "97|61", "97|47", "75|29", "61|13", "75|53", "29|13", "97|29",
            "53|29", "61|53", "97|53", "61|29", "47|13", "75|47", "97|75", "47|61", "75|61",
            "47|29", "75|13", "53|13",
        ]
        .into_iter()
        .map(String::from)
        .collect();
        let expected: HashMap<i32, Vec<i32>> = [
            (47, vec![53, 13, 61, 29]),
            (97, vec![13, 61, 47, 29, 53, 75]),
            (75, vec![29, 53, 47, 61, 13]),
            (61, vec![13, 53, 29]),
            (29, vec![13]),
            (53, vec![29, 13]),
        ]
        .iter()
        .cloned()
        .collect();
        let mut result_ruleset = Ruleset::new();
        for rule_str in input {
            result_ruleset.insert_rule_string(rule_str);
        }
        assert_eq!(result_ruleset.rules, expected);
    }

    #[test]
    fn test_orders() {
        let rule_set = Ruleset::new_from_map(
            [
                (47, vec![53, 13, 61, 29]),
                (97, vec![13, 61, 47, 29, 53, 75]),
                (75, vec![29, 53, 47, 61, 13]),
                (61, vec![13, 53, 29]),
                (29, vec![13]),
                (53, vec![29, 13]),
            ]
            .iter()
            .cloned()
            .collect(),
        );

        let input: Vec<(i32, i32)> = vec![
            (75, 53),
            (75, 95),
            (13, 29),
            (47, 61),
            (61, 53),
            (53, 29),
            (13, 75),
        ];
        let expected: Vec<bool> = vec![true, false, false, true, true, true, false];

        for (i, input_val) in input.iter().enumerate() {
            assert_eq!(
                rule_set.check_order(&input_val.0, &input_val.1),
                expected[i]
            );
        }
    }

    #[test]
    fn test_ordered_list() {
        let rule_set = Ruleset::new_from_map(
            [
                (47, vec![53, 13, 61, 29]),
                (97, vec![13, 61, 47, 29, 53, 75]),
                (75, vec![29, 53, 47, 61, 13]),
                (61, vec![13, 53, 29]),
                (29, vec![13]),
                (53, vec![29, 13]),
            ]
            .iter()
            .cloned()
            .collect(),
        );
        let input: Vec<Vec<i32>> = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];
        let expected: Vec<bool> = vec![true, true, true, false, false, false];

        for (i, input_val) in input.iter().enumerate() {
            assert_eq!(check_list(&input_val, &rule_set), expected[i]);
        }
    }

    #[test]
    fn test_solution_one() {
        let input = String::from(
            "47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13
        
        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47",
        );
        let expected = 143;

        assert_eq!(solution_one(&input), expected);
    }

    #[test]
    fn test_correct_list() {
        let rule_set = Ruleset::new_from_map(
            [
                (47, vec![53, 13, 61, 29]),
                (97, vec![13, 61, 47, 29, 53, 75]),
                (75, vec![29, 53, 47, 61, 13]),
                (61, vec![13, 53, 29]),
                (29, vec![13]),
                (53, vec![29, 13]),
            ]
            .iter()
            .cloned()
            .collect(),
        );
        let input: Vec<Vec<i32>> = vec![
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];
        let expected: Vec<Vec<i32>> = vec![
            vec![97, 75, 47, 61, 53],
            vec![61, 29, 13],
            vec![97, 75, 47, 29, 13],
        ];
        for (i, input_val) in input.iter().enumerate() {
            assert_eq!(correct_list(&input_val, &rule_set), expected[i]);
        }
    }

    #[test]
    fn test_solution_two() {
        let input = String::from(
            "47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13
        
        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47",
        );
        let expected = 123;

        assert_eq!(solution_two(&input), expected);
    }
}
