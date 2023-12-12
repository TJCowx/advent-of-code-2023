use core::fmt;
use std::collections::HashSet;

use crate::file_reader::file_reader;

static NEW_COLS: u64 = 999_999;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Galaxy(u64, u64);

impl Galaxy {
    fn new(x: usize, y: usize, empty_x: &Vec<u64>, empty_y: &Vec<u64>) -> Self {
        // Get number of x values in empty_x that are less than x
        let x_less_than = empty_x.iter().filter(|&val| *val < x as u64).count() as u64;
        let y_less_than = empty_y.iter().filter(|&val| *val < y as u64).count() as u64;

        let x_modified = (NEW_COLS * x_less_than) + x as u64;
        let y_modified = (NEW_COLS * y_less_than) + y as u64;

        println!(
            "VAL: ({}, {}), Item Count: ({},{}), Modified: ({},{})",
            x, y, x_less_than, y_less_than, x_modified, y_modified
        );

        Galaxy(x_modified, y_modified)
    }

    fn get_distance(&self, other: &Galaxy) -> u64 {
        self.0.abs_diff(other.0) as u64 + self.1.abs_diff(other.1) as u64
    }
}

#[derive(Debug)]
struct Universe {
    grid: Vec<Vec<char>>,
    galaxies: HashSet<Galaxy>,
    empty_x_vals: Vec<u64>,
    empty_y_vals: Vec<u64>,
}

impl Universe {
    fn create(lines: Vec<String>) -> Self {
        Universe {
            grid: lines.iter().map(|line| line.chars().collect()).collect(),
            galaxies: HashSet::new(),
            empty_x_vals: Vec::new(),
            empty_y_vals: Vec::new(),
        }
    }

    fn expand(&mut self) {
        for y in 0..self.grid.len() {
            // Check if all items in row contain only '.'
            if self.grid[y].iter().all(|&val| val == '.') {
                println!("Adding {} to empty_y_vals", y);
                self.empty_y_vals.push(y as u64);
            }
        }

        for x in 0..self.grid[0].len() {
            // Check if all items in col contain only '.'
            if self.grid.iter().all(|row| row[x] == '.') {
                println!("Adding {} to empty_x_vals", x);
                self.empty_x_vals.push(x as u64);
            }
        }
    }

    fn assign_galaxies(&mut self) {
        for row in 0..self.grid.len() {
            for col in 0..self.grid[0].len() {
                if self.grid[row][col] == '#' {
                    self.galaxies.insert(Galaxy::new(
                        col,
                        row,
                        &self.empty_x_vals,
                        &self.empty_y_vals,
                    ));
                }
            }
        }
    }

    fn get_distances_sum(&self) -> u64 {
        let mut sum = 0;

        for (i, galaxy) in self.galaxies.iter().enumerate() {
            for other in self.galaxies.iter().skip(i + 1) {
                sum += galaxy.get_distance(other);
            }
        }

        sum
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.grid.iter() {
            write!(f, "{}", row.iter().collect::<String>())?;
            write!(f, "\n")?;
        }
        Ok(())
    }
}

pub fn solve() {
    let lines = file_reader::read_as_vec("inputs/day11.txt");

    let mut universe = Universe::create(lines);

    universe.expand();

    universe.assign_galaxies();
    println!(
        "The universe is expanding... we have found the galaxies, there are {} of them!!!\n",
        universe.galaxies.len()
    );

    println!("Wow the universe is big, let's calculate the distances between the galaxies...\n");
    let distance = universe.get_distances_sum();

    println!("The distance between the galaxies is {}", distance);
}
