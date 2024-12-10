use std::collections::HashMap;
use std::ops::{Add, Sub};

struct AntennaGrid {
    map: Vec<Vec<char>>,
    antenna_map: HashMap<char, Vec<Point2D>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point2D {
    x: i32,
    y: i32,
}

impl Add for Point2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Point2D {
    fn new(coords: &(i32, i32)) -> Self {
        Self {
            x: coords.0,
            y: coords.1,
        }
    }
}

impl AntennaGrid {
    fn new(input: &String) -> Self {
        let mut grid: Vec<Vec<char>> = vec![];

        input
            .lines()
            .for_each(|line| grid.push(line.chars().collect()));

        Self {
            map: grid,
            antenna_map: HashMap::new(),
        }
    }

    fn height(&self) -> i32 {
        if self.map.len() == 0 {
            panic!("Map is empty");
        }
        self.map.len() as i32
    }

    fn width(&self) -> i32 {
        if self.map.len() == 0 {
            panic!("Map is empty");
        }
        if self.map[0].len() == 0 {
            panic!("Rows are empty");
        }
        self.map[0].len() as i32
    }

    // Put (0,0) in the bottom left of the grid and (width(), height()) in the top right
    fn convert_coords_readable(&self, coords: &(i32, i32)) -> Point2D {
        Point2D::new(&(coords.1, self.height() - 1 - coords.0))
    }

    // Convert back for accessing the map
    fn convert_coords_map(&self, coords: &Point2D) -> (i32, i32) {
        (self.height() - 1 - coords.y, coords.x)
    }

    fn initialise_antennae(&mut self) {
        for (i, line) in self.map.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if *c == '.' {
                    continue;
                }
                let new_coords = self.convert_coords_readable(&(i as i32, j as i32));
                self.antenna_map
                    .entry(*c)
                    .or_insert_with(Vec::new)
                    .push(new_coords);
            }
        }
    }

    fn count_anti_nodes(&self) -> i32 {
        let mut seen_antennae: Vec<Point2D> = vec![];
        for antenna in self.antenna_map.keys() {
            let antenna_locations = &self.antenna_map[antenna];
            for i in 0..antenna_locations.len() {
                for j in 0..antenna_locations.len() {
                    if i == j {
                        continue;
                    }
                    let anti_node =
                        self.find_anti_node(&antenna_locations[i], &antenna_locations[j]);
                    if self.in_bounds(&anti_node) && !seen_antennae.contains(&anti_node) {
                        seen_antennae.push(anti_node);
                    }
                }
            }
        }
        seen_antennae.len() as i32
    }

    fn count_resonant_anti_nodes(&self) -> i32 {
        let mut seen_antennae: Vec<Point2D> = vec![];
        for antenna in self.antenna_map.keys() {
            let antenna_locations = &self.antenna_map[antenna];
            // Resonant locations on top of the anttenae
            // Can't happen with only one
            if antenna_locations.len() > 1 {
                for antenna in antenna_locations {
                    // Could be seen already
                    if !seen_antennae.contains(antenna) {
                        seen_antennae.push(*antenna);
                    }
                }
            }
            for i in 0..antenna_locations.len() {
                for j in 0..antenna_locations.len() {
                    if i == j {
                        continue;
                    }
                    let anti_nodes =
                        self.find_anti_nodes(&antenna_locations[i], &antenna_locations[j]);
                    for anti_node in anti_nodes {
                        if !seen_antennae.contains(&anti_node) {
                            seen_antennae.push(anti_node);
                        }
                    }
                }
            }
        }
        seen_antennae.len() as i32
    }

    fn find_anti_nodes(&self, first: &Point2D, second: &Point2D) -> Vec<Point2D> {
        let mut results: Vec<Point2D> = vec![];
        let anti_node_diff = *second - *first;
        let mut anti_node = *second + anti_node_diff;
        while self.in_bounds(&anti_node) {
            results.push(anti_node.clone());
            anti_node = anti_node + anti_node_diff;
        }
        results
    }

    fn find_anti_node(&self, first: &Point2D, second: &Point2D) -> Point2D {
        *second + *second - *first
    }

    fn in_bounds(&self, point: &Point2D) -> bool {
        point.x >= 0 && point.x < self.width() && point.y >= 0 && point.y < self.height()
    }
}

pub fn solution_one(input: &String) -> i32 {
    let mut map = AntennaGrid::new(&input);
    map.initialise_antennae();
    map.count_anti_nodes()
}
pub fn solution_two(input: &String) -> i32 {
    let mut map = AntennaGrid::new(&input);
    map.initialise_antennae();
    map.count_resonant_anti_nodes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_one() {
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
        let expected = 14;
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
        assert_eq!(solution_two(&input), expected);
    }
}
