use helpers::{Grid, Point2D};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct ReindeerMap {
    grid: Grid<i32>,
    start_points: Vec<Point2D>,
    end_points: Vec<Point2D>,
    scores: HashMap<i32, i32>,
}

impl ReindeerMap {
    fn new(input: &String) -> Self {
        let mut start_points: Vec<Point2D> = vec![];
        let mut end_points: Vec<Point2D> = vec![];
        let grid = Grid::new(input);

        for (j, line) in grid.iter().enumerate() {
            for (i, value) in line.iter().enumerate() {
                let point: Point2D = grid.convert_coords_readable(&(i as i32, j as i32));
                if *grid.access_grid(&point) == 0 {
                    start_points.push(point);
                } else if *grid.access_grid(&point) == 9 {
                    end_points.push(point);
                }
            }
        }

        Self {
            grid,
            start_points,
            end_points,
            scores: HashMap::new(),
        }
    }

    // Goes from every end point and if it reaches a valid point it will increase that locations score by one
    fn assign_scores(&mut self) {
        for nine in self.end_points.clone() {
            self.step_and_branch(&nine, &Direction::Up, &true);
        }
    }

    fn collect_scores(&self) -> i32 {
        let mut result = 0;
        for zero in &self.start_points {
            let id = self.grid.point_to_int(zero);
            match self.scores.get(&id) {
                Some(val) => {
                    dbg!(&zero);
                    dbg!(&id);
                    result += val;
                }
                _ => panic!(),
            }
        }
        result
    }

    fn can_step(&self, cur_loc: &Point2D, dir: &Direction) -> Result<Point2D, &str> {
        let next_pos = match dir {
            Direction::Up => *cur_loc + Point2D::new(&(0, 1)),
            Direction::Right => *cur_loc + Point2D::new(&(1, 0)),
            Direction::Left => *cur_loc + Point2D::new(&(-1, 0)),
            Direction::Down => *cur_loc + Point2D::new(&(0, -1)),
        };
        if !self.grid.in_bounds(&next_pos) {
            return Err("Next point out of bounds");
        }

        let cur_height = self.grid.access_grid(&cur_loc);
        if *self.grid.access_grid(&next_pos) != cur_height - 1 {
            return Err("Next pos is too low");
        }
        return Ok(next_pos);
    }

    fn step_and_branch(&mut self, position: &Point2D, from: &Direction, start: &bool) {
        *self
            .scores
            .entry(self.grid.point_to_int(position))
            .or_insert(0) += 1;
        let dirs = vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];
        if *start {
            dirs.iter().filter_map(|dir| Some(*dir != *from));
        }
        for dir in dirs {
            if let Ok(next_pos) = self.can_step(&position, &dir) {
                self.step_and_branch(&next_pos, &dir, &false);
            };
        }
    }
}

pub fn solution_one(input: &String) -> i32 {
    let mut map = ReindeerMap::new(&input);
    map.assign_scores();
    map.collect_scores()
}
pub fn solution_two(input: &String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_one() {
        let input = String::from(
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        );
        let expected = 36;
        assert_eq!(solution_one(&input), expected);
    }

    #[test]
    fn test_solution_two() {
        let input = String::from(
            "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
        );
        let expected = 34;
        //assert_eq!(solution_two(&input), expected);
    }
}
