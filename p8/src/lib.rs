use helpers::{Grid, Point2D};
use std::collections::HashMap;

struct AntennaGrid {
    grid: Grid<char>,
    map: HashMap<char, Vec<Point2D>>,
}

impl AntennaGrid {
    fn new(input: &String) -> Self {
        Self {
            grid: Grid::<char>::new(input),
            map: HashMap::new(),
        }
    }

    fn initialise_antennae(&mut self) {
        for (i, line) in self.grid.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if *c == '.' {
                    continue;
                }
                let new_coords = self.grid.convert_coords_readable(&(i as i32, j as i32));
                self.map.entry(*c).or_insert_with(Vec::new).push(new_coords);
            }
        }
    }

    fn count_anti_nodes(&self) -> i32 {
        let mut seen_antennae: Vec<Point2D> = vec![];
        for antenna in self.map.keys() {
            let antenna_locations = &self.map[antenna];
            for i in 0..antenna_locations.len() {
                for j in 0..antenna_locations.len() {
                    if i == j {
                        continue;
                    }
                    let anti_node =
                        self.find_anti_node(&antenna_locations[i], &antenna_locations[j]);
                    if self.grid.in_bounds(&anti_node) && !seen_antennae.contains(&anti_node) {
                        seen_antennae.push(anti_node);
                    }
                }
            }
        }
        seen_antennae.len() as i32
    }

    fn count_resonant_anti_nodes(&self) -> i32 {
        let mut seen_antennae: Vec<Point2D> = vec![];
        for antenna in self.map.keys() {
            let antenna_locations = &self.map[antenna];
            // Resonant locations on top of the antenae
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
        while self.grid.in_bounds(&anti_node) {
            results.push(anti_node.clone());
            anti_node = anti_node + anti_node_diff;
        }
        results
    }

    fn find_anti_node(&self, first: &Point2D, second: &Point2D) -> Point2D {
        *second + *second - *first
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
