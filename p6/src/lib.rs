use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Location {
    Visited,
    Obstacle,
    Open,
    OutOfBounds,
}

#[derive(Debug, PartialEq, Clone)]
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
    cyclic: bool,
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
            cyclic: false,
        }
    }

    fn obstruct(&mut self, position: (i32, i32)) {
        self.grid[position.1 as usize][position.0 as usize] = Location::Obstacle
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

    fn get_visited(&self) -> Vec<(i32, i32)> {
        let mut result: Vec<(i32, i32)> = vec![];
        for (y, row) in self.grid.iter().enumerate() {
            for (x, location) in row.iter().enumerate() {
                match location {
                    Location::Visited => result.push((x as i32, y as i32)),
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
        let mut cycle_map: HashMap<(i32, i32), Vec<Direction>> = HashMap::new();
        while !self.out_of_bounds() {
            // If the direction at a seen position has already occured then we are on the same path and have a cycle
            let seen_directions: &mut Vec<Direction> =
                cycle_map.entry(self.guard_pos).or_insert_with(Vec::new);

            if seen_directions.contains(&self.direction) {
                self.cyclic = true;
                break;
            } else {
                seen_directions.push(self.direction.clone());
            }

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

    fn cyclic(&self) -> bool {
        self.cyclic
    }

    fn get_guard_pos(&self) -> (i32, i32) {
        self.guard_pos
    }
}

pub fn solution_one(input: &String) -> i32 {
    let mut map = Map::new(input);
    map.resolve_map();
    map.get_visited().len() as i32
}

pub fn solution_two(input: &String) -> i32 {
    let mut result = 0;
    let mut map_initial = Map::new(input);
    let initial_guard_pos = map_initial.get_guard_pos();
    map_initial.resolve_map();

    // Will panic if we start on an obstructed square
    let visited_squares: Vec<(i32, i32)> = map_initial
        .get_visited()
        .into_iter()
        .filter(|pos| *pos != initial_guard_pos)
        .collect();

    // Only visited squares will hit a new obstacle
    for visited in visited_squares {
        let mut new_map = Map::new(input);
        new_map.obstruct(visited);
        new_map.resolve_map();
        if new_map.cyclic() {
            result += 1;
        }
    }
    result
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
    }

    #[test]
    fn test_solution_two() {
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

        let expected = 6;

        assert_eq!(solution_two(&input), expected);
    }
}
