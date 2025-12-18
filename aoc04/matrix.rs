use std::convert::TryFrom;
use std::ops::Index;

type T = bool;

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
}

impl Index<(usize, usize)> for Matrix {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.vec[row * self._w + col]
    }
}
