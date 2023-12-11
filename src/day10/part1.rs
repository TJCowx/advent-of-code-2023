use std::collections::HashMap;

use crate::file_reader::file_reader;

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

#[derive(Debug, PartialEq, Eq)]
struct Coord {
    x: i16,
    y: i16,
}

impl Coord {
    fn new(x: i16, y: i16, max_x: &i16, max_y: &i16) -> Option<Coord> {
        if x > *max_x || y > *max_y {
            println!("Invalid coord: {}, {}, MAX({}, {})", x, y, max_x, max_y);
            return None;
        }
        Some(Coord { x, y })
    }
}

#[derive(Debug)]
struct Pipe {
    position: Coord,
    is_start: bool,
    adj_pos_1: Coord,
    adj_pos_2: Coord,
}

impl Pipe {
    fn new(c: &char, position: Coord, max_x: i16, max_y: i16) -> Option<Pipe> {
        let in_position = Pipe::get_in_position(c, &position, &max_x, &max_y);
        let out_position = Pipe::get_out_position(c, &position, &max_x, &max_y);

        if in_position.is_none() || out_position.is_none() {
            println!("Invalid pipe at {:?}, {}", position, c);
            return None;
        }

        Some(Pipe {
            position,
            is_start: false,
            adj_pos_1: in_position.unwrap(),
            adj_pos_2: out_position.unwrap(),
        })
    }

    fn get_id(&self) -> String {
        if self.is_start {
            return 'S'.to_string();
        }
        format!("{}-{}", self.position.x, self.position.y)
    }

    fn is_valid(&self, max_x: i16, max_y: i16) -> bool {
        self.adj_pos_1.x <= max_x
            && self.adj_pos_1.y <= max_y
            && self.adj_pos_1.y >= 0
            && self.adj_pos_1.x >= 0
            && self.adj_pos_2.x <= max_x
            && self.adj_pos_2.y <= max_y
            && self.adj_pos_2.x >= 0
            && self.adj_pos_2.y >= 0
    }

    fn is_adj_coord(&self, other: &Coord) -> bool {
        self.adj_pos_1 == *other || self.adj_pos_2 == *other
    }

    fn can_connect(&self, other: &Pipe) -> bool {
        // if the other pipe has an adj_coordinate that is this pipe's position, then it can connect and reverse
        (other.adj_pos_1 == self.position || other.adj_pos_2 == self.position)
            && (self.adj_pos_1 == other.position || self.adj_pos_2 == other.position)
    }

    fn get_next_position(&self, prev: &Coord) -> Option<&Coord> {
        if self.adj_pos_1 == *prev {
            return Some(&self.adj_pos_2);
        }
        if self.adj_pos_2 == *prev {
            return Some(&self.adj_pos_1);
        }
        None
    }

    fn get_in_position(c: &char, pos: &Coord, max_x: &i16, max_y: &i16) -> Option<Coord> {
        match c {
            '|' => Coord::new(pos.x, pos.y - 1, max_x, max_y),
            '-' => Coord::new(pos.x - 1, pos.y, max_x, max_y),
            'L' => Coord::new(pos.x + 1, pos.y, max_x, max_y),
            'J' => Coord::new(pos.x, pos.y - 1, max_x, max_y),
            '7' => Coord::new(pos.x, pos.y + 1, max_x, max_y),
            'F' => Coord::new(pos.x + 1, pos.y, max_x, max_y),
            '.' => None,
            'S' => None,
            _ => panic!("Invalid pipe char, {}", c.to_string()),
        }
    }

    fn get_out_position(c: &char, pos: &Coord, max_x: &i16, max_y: &i16) -> Option<Coord> {
        match c {
            '|' => Coord::new(pos.x, pos.y + 1, max_x, max_y),
            '-' => Coord::new(pos.x + 1, pos.y, max_x, max_y),
            'L' => Coord::new(pos.x, pos.y - 1, max_x, max_y),
            'J' => Coord::new(pos.x - 1, pos.y, max_x, max_y),
            '7' => Coord::new(pos.x - 1, pos.y, max_x, max_y),
            'F' => Coord::new(pos.x, pos.y + 1, max_x, max_y),
            '.' => None,
            'S' => None,
            _ => panic!("Invalid pipe char, {}", c.to_string()),
        }
    }
}

#[derive(Debug)]
struct Board {
    pipes: HashMap<String, Pipe>,
    start: Coord,
    max_x: i16,
    max_y: i16,
}

impl Board {
    fn new() -> Board {
        Board {
            pipes: HashMap::new(),
            start: Coord { x: 0, y: 0 },
            max_x: 0,
            max_y: 0,
        }
    }

    fn add_pipe(&mut self, pipe: Pipe) {
        self.pipes.insert(pipe.get_id(), pipe);
    }

    fn get_pipe(&self, coord: &Coord) -> Option<&Pipe> {
        self.pipes.get(&format!("{}-{}", coord.x, coord.y))
    }

    fn build_board(&mut self, lines: &Vec<String>) {
        self.max_x = lines[0].len() as i16;
        self.max_y = lines.len() as i16;

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '.' {
                    continue;
                }

                let coord = Coord::new(x as i16, y as i16, &self.max_x, &self.max_y).unwrap();
                if c == 'S' {
                    self.start = coord;
                    continue;
                }

                let pipe = Pipe::new(&c, coord, self.max_x, self.max_y);

                match pipe {
                    Some(p) => {
                        if p.is_valid(self.max_x, self.max_y) {
                            self.add_pipe(p);
                        } else {
                            continue;
                        }
                    }
                    None => continue,
                }
            }
        }
    }

    fn get_start_paths(&self) -> (Vec<&Pipe>, HashMap<String, &Coord>) {
        let mut paths = Vec::new();
        let mut path_prev_hash = HashMap::new();

        let directions = vec![
            (
                "north",
                Coord::new(self.start.x, self.start.y - 1, &self.max_x, &self.max_y),
            ),
            (
                "south",
                Coord::new(self.start.x, self.start.y + 1, &self.max_x, &self.max_y),
            ),
            (
                "east",
                Coord::new(self.start.x + 1, self.start.y, &self.max_x, &self.max_y),
            ),
            (
                "west",
                Coord::new(self.start.x - 1, self.start.y, &self.max_x, &self.max_y),
            ),
        ];

        for (direction, coord) in directions {
            if let Some(pipe) = coord
                .and_then(|coord| self.get_pipe(&coord))
                .filter(|pipe| pipe.is_adj_coord(&self.start))
            {
                paths.push(pipe);
                path_prev_hash.insert(pipe.get_id(), &self.start);
            } else {
                println!("Can't go {} from start", direction);
            }
        }

        (paths, path_prev_hash)
    }

    fn solve_path(&mut self) {
        let (mut paths, mut prev_steps) = self.get_start_paths();

        let mut mid_found = false;
        let mut steps_taken = 1;

        while !mid_found {
            steps_taken += 1;
            let mut new_paths = Vec::new();
            // Get the next pipe from the step, make sure it isn't the previous
            // pipe, and that it isn't the start pipe
            for path in paths {
                // Get the next step, not in the prev
                let next_step =
                    match path.get_next_position(prev_steps.get(&path.get_id()).unwrap()) {
                        Some(next) => next,
                        None => continue,
                    };

                // Get the next pipe
                let next_pipe = match self.get_pipe(next_step) {
                    Some(pipe) => pipe,
                    None => continue,
                };

                // Check if the next pipe can connect to the current pipe
                if path.can_connect(next_pipe) {
                    new_paths.push(next_pipe);
                    prev_steps.insert(next_pipe.get_id(), &path.position);
                }
            }

            // Check if there are two paths on the same coordinate
            mid_found = new_paths
                .iter()
                .filter(|pipe| {
                    new_paths
                        .iter()
                        .filter(|other| pipe.position == other.position)
                        .count()
                        > 1
                })
                .count()
                > 0;

            paths = new_paths;
        }

        println!("Steps taken: {}", steps_taken);
    }
}

pub fn solve() {
    let lines = file_reader::read_as_vec("inputs/day10.txt");

    let mut board = Board::new();
    board.build_board(&lines);

    println!("Start: {:?}", board.start);

    board.solve_path();

    println!("Day 10 Part 1");
}
