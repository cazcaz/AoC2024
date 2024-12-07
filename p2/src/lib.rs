use test_helpers;
struct IncreasingTracker {
    set: bool,
    increasing: bool,
}

pub fn validate_list(list: &Vec<i32>) -> bool {
    let list_boundary = create_boundary(list);
    validate_boundary(&list_boundary)
}

pub fn validate_list_with_damp(list: &Vec<i32>) -> bool {
    if validate_list(&list) {
        return true;
    }
    for (i, _) in list.iter().enumerate() {
        let i_cast = i as i32;
        let damped_list = dampen_level(&i_cast, &list);
        if validate_list(&damped_list) {
            return true;
        }
    }
    false
}

fn validate_boundary(list: &Vec<i32>) -> bool {
    let mut tracker = IncreasingTracker {
        set: false,
        increasing: false,
    };
    for i in list {
        if *i == 0 {
            return false;
        } else if i.abs() > 3 {
            return false;
        }
        if tracker.set {
            if tracker.increasing != (*i > 0) {
                return false;
            }
        } else {
            tracker.increasing = *i > 0;
            tracker.set = true;
        }
    }
    true
}

fn create_boundary(list: &Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    for i in 0..list.len() - 1 {
        result.push(list[i + 1] - list[i]);
    }
    result
}

fn dampen_level(damped_level: &i32, list: &Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    for (i, level) in list.iter().enumerate() {
        if i as i32 == *damped_level {
            continue;
        }
        result.push(*level);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_lists_with_damp() {
        let input: Vec<Vec<i32>> = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        let expected = vec![true, false, false, true, true, true];
        test_helpers::test_function(input, expected, validate_list_with_damp);
    }

    #[test]
    fn test_validate_lists() {
        let input: Vec<Vec<i32>> = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        let expected = vec![true, false, false, false, false, true];
        test_helpers::test_function(input, expected, validate_list);
    }

    #[test]
    fn test_list_boundary() {
        let input: Vec<Vec<i32>> = vec![
            vec![1, 2, 3, 4, 5],
            vec![1, 10, -5, 4, 2, 6],
            vec![1, 3, 3, 1, 3, 2, 1],
            vec![1, 1, 1, 1, 1],
            vec![-2, 3, 2, 1, 0],
        ];
        let expected: Vec<Vec<i32>> = vec![
            vec![1, 1, 1, 1],
            vec![9, -15, 9, -2, 4],
            vec![2, 0, -2, 2, -1, -1],
            vec![0, 0, 0, 0],
            vec![5, -1, -1, -1],
        ];
        test_helpers::test_function(input, expected, create_boundary);
    }

    #[test]
    fn test_validate_boundary() {
        let input: Vec<Vec<i32>> = vec![
            vec![1, 1, 1, 1],
            vec![9, -15, 9, -3, 4],
            vec![0, 0, 0, 3, 3, 2, 1, 3],
            vec![5, 2, -1, -2],
            vec![1, 2, 3, 2, 1],
            vec![1, 2, 3, 1, 2, -1],
            vec![-1, -2, -1, -3],
        ];
        let expected: Vec<bool> = vec![true, false, false, false, true, false, true];
        test_helpers::test_function(input, expected, validate_boundary);
    }

    #[test]
    fn test_dampen_level() {
        fn unwrap_dampen(input: &(i32, Vec<i32>)) -> Vec<i32> {
            dampen_level(&input.0, &input.1)
        }

        let input: Vec<(i32, Vec<i32>)> = vec![
            (1, vec![1, 1, 1, 1]),
            (3, vec![9, -15, 9, -3, 4]),
            (4, vec![0, 0, 0, 3, 3, 2, 1, 3]),
            (0, vec![5, 2, -1, -2]),
            (4, vec![1, 2, 3, 2, 1]),
            (5, vec![1, 2, 3, 1, 2, -1]),
            (1, vec![-1, -2, -1, -3]),
        ];
        let expected: Vec<Vec<i32>> = vec![
            vec![1, 1, 1],
            vec![9, -15, 9, 4],
            vec![0, 0, 0, 3, 2, 1, 3],
            vec![2, -1, -2],
            vec![1, 2, 3, 2],
            vec![1, 2, 3, 1, 2],
            vec![-1, -1, -3],
        ];
        test_helpers::test_function(input, expected, unwrap_dampen);
    }
}
