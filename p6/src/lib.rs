#[derive(Clone, Debug)]
enum Location {
    Visited,
    Obstacle,
    Open,
    OutOfBounds,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Map {
    guard_pos: (i32, i32),
    grid: Vec<Vec<Location>>,
    direction: Direction,
}

impl Map {
    fn new(input: &String) -> Self {
        let mut grid: Vec<Vec<Location>> = vec![];
        let mut direction: Direction = Direction::Up;
        let mut pos: (i32, i32) = (0, 0);

        for (i, line) in input.lines().enumerate() {
            let mut current_line: Vec<Location> = vec![];
            for (j, c) in line.chars().enumerate() {
                match c {
                    '.' => current_line.push(Location::Open),
                    '#' => current_line.push(Location::Obstacle),
                    '^' => {
                        current_line.push(Location::Open);
                        direction = Direction::Up;
                        pos = (j as i32, i as i32);
                    }
                    '>' => {
                        current_line.push(Location::Open);
                        direction = Direction::Right;
                        pos = (j as i32, i as i32);
                    }
                    'v' => {
                        current_line.push(Location::Open);
                        direction = Direction::Down;
                        pos = (j as i32, i as i32);
                    }
                    '<' => {
                        current_line.push(Location::Open);
                        direction = Direction::Left;
                        pos = (j as i32, i as i32);
                    }
                    _ => {}
                }
            }
            grid.push(current_line);
        }

        Self {
            grid,
            direction,
            guard_pos: pos,
        }
    }

    fn out_of_bounds(&self) -> bool {
        let (y, x) = self.guard_pos;
        let grid_height = self.grid.len() as i32;
        if grid_height == 0 {
            panic!("Map is empty");
        }
        let grid_width = self.grid[0].len() as i32;
        if grid_width == 0 {
            panic!("Map rows are empty");
        }
        y >= grid_height || x >= grid_width || y < 0 || x < 0
    }

    fn rotate_direction(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }

    fn move_guard(&self) -> (i32, i32) {
        match self.direction {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }

    fn search_ahead(&self) -> Location {
        let next_move = self.move_guard();
        let next_pos = (
            self.guard_pos.0 + next_move.0,
            self.guard_pos.1 + next_move.1,
        );
        if next_pos.1 < self.grid.len() as i32
            && next_pos.0 < self.grid[0].len() as i32
            && next_pos.0 >= 0
            && next_pos.1 >= 0
        {
            self.grid[next_pos.1 as usize][next_pos.0 as usize].clone()
        } else {
            Location::OutOfBounds
        }
    }

    fn step_forward(&mut self) {
        let next_move = self.move_guard();
        self.guard_pos = (
            self.guard_pos.0 + next_move.0,
            self.guard_pos.1 + next_move.1,
        );
    }

    fn count_visited(&self) -> i32 {
        let mut result = 0;
        for row in &self.grid {
            for location in row {
                match location {
                    Location::Visited => result += 1,
                    _ => continue,
                }
            }
        }
        result
    }

    fn current_pos(&self) -> Location {
        self.grid[self.guard_pos.1 as usize][self.guard_pos.0 as usize].clone()
    }

    fn resolve_map(&mut self) {
        while !self.out_of_bounds() {
            match self.current_pos() {
                Location::Open => {
                    self.grid[self.guard_pos.1 as usize][self.guard_pos.0 as usize] =
                        Location::Visited;
                }
                Location::Visited => {}
                Location::Obstacle | Location::OutOfBounds => {
                    panic!();
                }
            }
            while true {
                match self.search_ahead() {
                    Location::Obstacle => self.rotate_direction(),
                    Location::Open | Location::Visited | Location::OutOfBounds => break,
                }
            }
            self.step_forward();
        }
    }
}

pub fn solution_one(input: &String) -> i32 {
    let mut map = Map::new(input);
    map.resolve_map();
    map.count_visited()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_one() {
        let input = String::from(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        );

        let expected = 41;

        assert_eq!(solution_one(&input), expected);
    }
}
