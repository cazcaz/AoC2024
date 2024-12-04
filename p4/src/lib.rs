pub fn search_xmas(grid: Vec<String>) -> i32 {
    let mut count = 0;
    let directions = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    for (y, line) in grid.iter().enumerate() {
        for (x, letter) in line.chars().enumerate() {
            if letter == 'X' {
                for &dir in &directions {
                    if find_xmas(&grid, (x as i32, y as i32), dir) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn find_xmas(grid: &Vec<String>, start: (i32, i32), dir: (i32, i32)) -> bool {
    let mut current_pos = start;
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;
    let chars: Vec<char> = "XMAS".chars().collect();
    for i in 0..4 {
        if current_pos.0 < 0
            || current_pos.0 >= width
            || current_pos.1 < 0
            || current_pos.1 >= height
        {
            return false;
        }
        if grid[current_pos.1 as usize]
            .chars()
            .nth(current_pos.0 as usize)
            .unwrap()
            != chars[i]
        {
            return false;
        }
        current_pos = (current_pos.0 + dir.0, current_pos.1 + dir.1);
    }
    true
}

pub fn search_x_mas(grid: Vec<String>) -> i32 {
    let mut count = 0;
    let directions = [
        (1, 1),
        (1, -1),
        (-1, -1),
        (-1, 1),
    ];
    for (y, line) in grid.iter().enumerate() {
        for (x, letter) in line.chars().enumerate() {
            if letter == 'A' {
                for &dir in &directions {
                    if find_x_mas(&grid, (x as i32, y as i32), dir) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn find_x_mas(grid: &Vec<String>, start: (i32, i32), dir: (i32, i32)) -> bool {
    let mut current_pos = (start.0 + dir.0, start.1 + dir.1);
    let mut current_dir = dir;
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;
    let chars: Vec<char> = "MMSS".chars().collect();
    for i in 0..4 {
        if current_pos.0 < 0
            || current_pos.0 >= width
            || current_pos.1 < 0
            || current_pos.1 >= height
        {
            return false;
        }
        if grid[current_pos.1 as usize]
            .chars()
            .nth(current_pos.0 as usize)
            .unwrap()
            != chars[i]
        {
            return false;
        }
        current_dir = rotate_dir(current_dir);
        current_pos = (start.0 + current_dir.0, start.1 + current_dir.1);
    }
    true
}

fn rotate_dir(dir: (i32, i32)) -> (i32, i32) {
    (dir.1 , - dir.0) 
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_search() {
        let input: Vec<String> = vec!["..X...", ".SAMX.", ".A..A.", "XMAS.S", ".X...."]
        .into_iter()
        .map(String::from)
        .collect();
        
        let expected = 4;
        assert_eq!(search_xmas(input), expected);
        
        let input2: Vec<String> = vec![
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
            ]
            .into_iter()
            .map(String::from)
            .collect();
            
            let expected2 = 18;
        assert_eq!(search_xmas(input2), expected2);
    }

    #[test]
    fn test_x_mas() {
        let input : Vec<String>= vec![
            ".M.S......",
            "..A..MSMS.",
            ".M.S.MAA..",
            "..A.ASMSM.",
            ".M.S.M....",
            "..........",
            "S.S.S.S.S.",
            ".A.A.A.A..",
            "M.M.M.M.M.",
            "..........",
        ].into_iter().map(String::from).collect();

        let expected = 9;
        assert_eq!(search_x_mas(input), expected);
    }

    #[test]
    fn test_rotate_dir() {
        let input1 = (1,2);
        let expected1 = (2,-1);

        let input2 = (3,3);
        let expected2 = (3,-3);

        let input3 = (0,0);
        let expected3 = (0,0);

        assert_eq!(rotate_dir(input1), expected1);
        assert_eq!(rotate_dir(input2), expected2);
        assert_eq!(rotate_dir(input3), expected3);
    }
}
