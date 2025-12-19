use std::convert::TryFrom;
use std::ops::{Index, IndexMut};

type T = bool;

#[derive(Clone)]
pub struct Matrix {
    vec: Vec<T>,
    _w: usize,
    _h: usize,
}

impl Matrix {
    pub fn new(w: usize) -> Matrix {
        Matrix {
            vec: vec![],
            _w: w,
            _h: 0,
        }
    }

    pub fn parse_line_default(line: &str, w: usize) -> Vec<T> {
        // returns a Vec with <= w elements
        return Matrix::parse_line(line, w, '@');
    }
    pub fn parse_line(line: &str, w: usize, true_char: char) -> Vec<T> {
        // returns a Vec with <= w elements
        let vec = line
            .trim()
            .chars()
            .take(w)
            .map(|ch| ch == true_char)
            .collect::<Vec<T>>();

        vec
    }

    pub fn width(&self) -> usize {
        self._w
    }

    pub fn height(&self) -> usize {
        self._h
    }

    fn add_row(&mut self, new_row: &mut Vec<T>) -> bool {
        // appends new_row as a new row to the matrix
        if new_row.len() == 0 {
            return false;
        }

        self._h += 1;
        self.vec.append(new_row);
        self.vec.resize(self._w * self._h, false); // ensure correct size, fill empty spaces

        true
    }

    pub fn set_height(&mut self, new_height: usize) {
        self._h = new_height;
        self.vec.resize(self._w * self._h, false);
    }

    pub fn push_line(&mut self, line: &str) -> bool {
        // parses line and adds it to the matrix IF not empty
        let mut line_vec = Matrix::parse_line_default(line, self._w);

        self.add_row(&mut line_vec)
    }

    pub fn get_line(&self, row: usize, false_char: char, true_char: char) -> Option<String> {
        // converts a row into the input format
        if row >= self._h {
            None
        } else {
            Some(
                self.vec
                    .iter()
                    .skip(row * self._w)
                    .take(self._w)
                    .map(|&b| if b { true_char } else { false_char })
                    .collect(),
            )
        }
    }

    pub fn get_last_line(&self, false_char: char, true_char: char) -> Option<String> {
        // converts the last row into the input format
        self.get_line(
            self._h.checked_sub(1).unwrap_or_default(),
            false_char,
            true_char,
        )
    }

    pub fn get(&self, (row, col): (usize, usize)) -> bool {
        // get copy of element at (row, col), or false if out of bounds
        if col >= self._w {
            return false;
        }
        if row >= self._h {
            return false;
        }

        return self[(row, col)];
    }

    pub fn get_coords(&self, (row, col): (i64, i64)) -> bool {
        // like get, but negative row numbers allowed
        let safe_row = match usize::try_from(row) {
            Ok(val) => val,
            Err(_) => return false,
        };
        let safe_col = match usize::try_from(col) {
            Ok(val) => val,
            Err(_) => return false,
        };

        return self.get((safe_row, safe_col));
    }

    pub fn get_rolls_nearby(&self, (row, col): (i64, i64)) -> usize {
        let mut count = 0;

        for y in -1..=1 {
            for x in -1..=1 {
                if self.get_coords((row + y, col + x)) {
                    count += 1;
                }
            }
        }
        if self.get_coords((row, col)) {
            count -= 1;
        }

        count
    }

    pub fn set(&mut self, (row, col): (usize, usize), new_val: T) {
        // set copy of element at (row, col), or false if out of bounds
        if col >= self._w {
            return;
        }
        if row >= self._h {
            return;
        }

        self[(row, col)] = new_val;
    }

    pub fn copy_remove_all_accessible(&mut self, mtx_out: &mut Matrix) -> usize {
        // finds all accessible rolls of paper (true values with at most 4 true neighbors) and removes them from mtx_out
        // returns the number of rolls removed
        assert_eq!(mtx_out.width(), self.width());
        assert_eq!(mtx_out.height(), self.height());

        let mut total = 0;
        for row in 0..self.height() {
            for col in 0..self.width() {
                if self.get((row, col)) && self.get_rolls_nearby((row as i64, col as i64)) < 4 {
                    mtx_out.set((row, col), false);
                    total += 1;
                }
            }
        }
        total
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.vec[row * self._w + col]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.vec[row * self._w + col]
    }
}
