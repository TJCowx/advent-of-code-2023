use core::fmt;
use std::collections::HashSet;

use crate::file_reader::file_reader;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Galaxy(usize, usize);

impl Galaxy {
    fn get_distance(&self, other: &Galaxy) -> u64 {
        self.0.abs_diff(other.0) as u64 + self.1.abs_diff(other.1) as u64
    }
}

#[derive(Debug)]
struct Universe {
    grid: Vec<Vec<char>>,
    galaxies: HashSet<Galaxy>,
}

impl Universe {
    fn create(lines: Vec<String>) -> Self {
        Universe {
            grid: lines.iter().map(|line| line.chars().collect()).collect(),
            galaxies: HashSet::new(),
        }
    }

    fn get_empty_cols(&self) -> Vec<usize> {
        let mut empty_cols = Vec::new();
        for col in 0..self.grid[0].len() {
            let mut is_empty = true;
            for row in 0..self.grid.len() {
                if self.grid[row][col] != '.' {
                    is_empty = false;
                    break;
                }
            }
            if is_empty {
                empty_cols.push(col);
            }
        }
        empty_cols
    }

    fn get_empty_rows(&self) -> Vec<usize> {
        let mut empty_rows = Vec::new();
        for row in 0..self.grid.len() {
            let mut is_empty = true;
            for col in 0..self.grid[0].len() {
                if self.grid[row][col] != '.' {
                    is_empty = false;
                    break;
                }
            }
            if is_empty {
                empty_rows.push(row);
            }
        }
        empty_rows
    }

    fn expand(&mut self) {
        let empty_cols = self.get_empty_cols();
        let empty_rows = self.get_empty_rows();

        for col in empty_cols.iter().rev() {
            for row in 0..self.grid.len() {
                self.grid[row].insert(*col, '.');
            }
        }

        for row in empty_rows.iter().rev() {
            self.grid.insert(*row, vec!['.'; self.grid[0].len()]);
        }
    }

    fn assign_galaxies(&mut self) {
        for row in 0..self.grid.len() {
            for col in 0..self.grid[0].len() {
                if self.grid[row][col] == '#' {
                    self.galaxies.insert(Galaxy(col, row));
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

    println!("The big bang has happened, the universe is expanding..\n");
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
