use std::cmp::max;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Matrix {
    data: HashMap<(isize, isize), i64>,
    max_x: isize,
    max_y: isize,
}

impl Matrix {
    pub fn new() -> Matrix {
        Matrix {
            ..Default::default()
        }
    }

    pub fn get(&self, x: isize, y: isize) -> Option<&i64> {
        self.data.get(&(x, y))
    }

    pub fn set(&mut self, x: isize, y: isize, value: i64) {
        *self.data.entry((x, y)).or_insert(0) = value;
        self.max_x = max(self.max_x, x);
        self.max_y = max(self.max_y, y);
    }

    pub fn dimensions(&self) -> (isize, isize) {
        (self.max_x, self.max_y)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn matrix() {
        let mut matrix = Matrix::new();
        matrix.set(1, 1, 1);
        let result = matrix.get(1, 1);
        assert_eq!(result, Some(&1i64));
    }
}
