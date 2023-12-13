use crate::file_reader::file_reader;

#[derive(Debug)]
struct Pattern {
    pattern: Vec<Vec<char>>,
}

impl Pattern {
    fn new(input: Vec<String>) -> Pattern {
        Pattern {
            pattern: input.iter().map(|x| x.chars().collect()).collect(),
        }
    }

    fn get_mirrored_cols(&self, start: usize) -> Vec<Vec<char>> {
        let mut cols = vec![];
        for i in start..self.pattern[0].len() {
            cols.push(self.get_column(i));
        }

        cols
    }

    fn get_orig_cols(&self, end: usize) -> Vec<Vec<char>> {
        let mut cols = vec![];
        for i in (0..=end).rev() {
            cols.push(self.get_column(i));
        }

        cols
    }

    fn get_column(&self, index: usize) -> Vec<char> {
        self.pattern.iter().map(|x| x[index]).collect()
    }

    fn get_row(&self, index: usize) -> Vec<char> {
        self.pattern[index].clone()
    }

    fn get_mirror_rows(&self, start: usize) -> Vec<Vec<char>> {
        let mut rows = vec![];
        for i in start..self.pattern.len() {
            rows.push(self.get_row(i));
        }

        rows
    }

    fn get_original_rows(&self, end: usize) -> Vec<Vec<char>> {
        let mut rows = vec![];
        for i in (0..=end).rev() {
            rows.push(self.get_row(i));
        }

        rows
    }

    fn find_vertical_reflection(&self) -> Option<usize> {
        // loop through each column
        'cols: for i in 0..self.pattern[0].len() - 1 {
            let column = self.get_column(i);
            let next_column = self.get_column(i + 1);

            // Are the columns the same?
            if column == next_column {
                let mut mirror_cols = self.get_mirrored_cols(i + 1);
                let orig_cols = self.get_orig_cols(i);

                if orig_cols.len() < mirror_cols.len() {
                    mirror_cols.truncate(orig_cols.len());
                }

                for (j, col) in mirror_cols.iter().enumerate() {
                    if col != &orig_cols[j] {
                        continue 'cols; // There isn't a reflection here, continue to next column
                    }
                }

                return Some(i + 1);
            }
        }

        None
    }

    fn find_horizontal_reflection(&self) -> Option<usize> {
        // loop through each row
        'rows: for i in 0..self.pattern.len() - 1 {
            let row = self.get_row(i);
            let next_row = self.get_row(i + 1);

            // Are the rows the same?
            if row == next_row {
                let mut mirror_rows = self.get_mirror_rows(i + 1);
                let orig_rows = self.get_original_rows(i);

                if orig_rows.len() < mirror_rows.len() {
                    // Lets trim any extra mirror rows
                    mirror_rows.truncate(orig_rows.len());
                }

                for (j, row) in mirror_rows.iter().enumerate() {
                    if row != &orig_rows[j] {
                        continue 'rows; // There isn't a reflection here, continue to next row
                    }
                }

                return Some(i + 1);
            }
        }

        None
    }
}

pub fn solve() {
    let lines = file_reader::read_as_vec("inputs/day13.txt");

    let mut sum = 0;

    let patterns = lines
        .split(|l| l.is_empty())
        .map(|g| Pattern::new(g.to_vec()))
        .collect::<Vec<Pattern>>();

    for p in patterns {
        println!("NEW PATTERN CHECK");
        if let Some(pos) = p.find_vertical_reflection() {
            sum += pos;
        }

        if let Some(pos) = p.find_horizontal_reflection() {
            sum += pos * 100;
        }
    }

    println!("Day 13, Part 1: {}", sum);
}
