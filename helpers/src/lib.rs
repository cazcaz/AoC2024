use std::ops::{Add, Sub};
use std::str::FromStr;

pub struct Grid<T> {
    grid: Vec<Vec<T>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
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
    pub fn new(coords: &(i32, i32)) -> Self {
        Self {
            x: coords.0,
            y: coords.1,
        }
    }
}

impl<T> Grid<T>
where
    T: FromStr + Default,
    <T as FromStr>::Err: std::fmt::Debug,
{
    pub fn iter(&self) -> impl Iterator<Item = &Vec<T>> {
        self.grid.iter()
    }

    pub fn new(input: &String) -> Self {
        let grid: Vec<Vec<T>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse().expect("Failed to parse"))
                    .collect()
            })
            .collect();

        Self { grid }
    }

    fn height(&self) -> i32 {
        if self.grid.is_empty() {
            panic!("Map is empty");
        }
        self.grid.len() as i32
    }

    fn width(&self) -> i32 {
        if self.grid.is_empty() {
            panic!("Map is empty");
        }
        if self.grid[0].is_empty() {
            panic!("Rows are empty");
        }
        self.grid[0].len() as i32
    }

    // Put (0,0) in the bottom left of the grid and (width(), height()) in the top right
    pub fn convert_coords_readable(&self, coords: &(i32, i32)) -> Point2D {
        Point2D::new(&(coords.1, self.height() - 1 - coords.0))
    }

    // Convert back for accessing the map
    fn convert_coords_grid(&self, coords: &Point2D) -> (i32, i32) {
        (self.height() - 1 - coords.y, coords.x)
    }

    pub fn access_grid(&self, coords: &Point2D) -> &T {
        let original_coords = self.convert_coords_grid(&coords);
        &self.grid[original_coords.0 as usize][original_coords.1 as usize]
    }

    pub fn in_bounds(&self, point: &Point2D) -> bool {
        point.x >= 0 && point.x < self.width() && point.y >= 0 && point.y < self.height()
    }

    // Hashable point
    pub fn point_to_int(&self, point: &Point2D) -> i32 {
        point.y * self.height() + point.x
    }
}
