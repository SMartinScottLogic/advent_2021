use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

use utils::Matrix;

pub fn load(filename: &str) -> Solution {
    let file = File::open(filename).unwrap();

    let reader = BufReader::new(file);

    let mut solution = Solution::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let line = Line::from_str(&line);
        solution.add(line.unwrap());
    }

    solution
}

#[derive(Debug, Default)]
pub struct Solution {
    points: Matrix,
    folds: Vec<(Direction, isize)>,
    answer: i64,
}

impl Solution {
    fn new() -> Self {
        Self {
            folds: Vec::new(),
            ..Default::default()
        }
    }

    fn add(&mut self, marks: Line) {
        match marks {
            Line::Point(x, y) => self.points.set(x as isize, y as isize, 1),
            Line::Fold(dirn, posn) => self.folds.push((dirn, posn)),
            _ => {}
        }
    }

    fn fold(&mut self, direction: Direction, position: isize) {
        let mut next = Matrix::new();
        let (xsize, ysize) = self.points.dimensions();
        for y in 0..=ysize {
            for x in 0..=xsize {
                if let Some(1) = self.points.get(x, y) {
                    match direction {
                        Direction::X if x < position => {
                            next.set(x, y, 1);
                        }
                        Direction::Y if y < position => {
                            next.set(x, y, 1);
                        }
                        Direction::X => {
                            next.set(position - (x - position), y, 1);
                        }
                        Direction::Y => {
                            next.set(x, position - (y - position), 1);
                        }
                    };
                }
            }
        }
        self.points = next;
    }

    fn count(&self) -> i32 {
        let mut count = 0;
        let (xsize, ysize) = self.points.dimensions();
        for y in 0..=ysize {
            for x in 0..=xsize {
                if let Some(1) = self.points.get(x, y) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn analyse(&mut self) {
        let (direction, position) = self.folds.get(0).unwrap();
        let direction = *direction;
        let position = *position;
        self.fold(direction, position);
        self.answer = self.count() as i64;
        println!("{}", self.answer);
    }

    pub fn answer(&self) -> i64 {
        self.answer as i64
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    X,
    Y,
}
enum Line {
    Point(i32, i32),
    Fold(Direction, isize),
    None,
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("line = {}", s);
        if s.starts_with("fold along") {
            println!("fold");
            let fold = s.to_owned().replace("fold along ", "");
            let mut fold = fold.split("=");
            let dirn = fold.next().unwrap();
            let posn = fold.next().unwrap().parse::<isize>()?;
            match dirn {
                "x" => Ok(Self::Fold(Direction::X, posn)),
                "y" => Ok(Self::Fold(Direction::Y, posn)),
                _ => dirn.parse::<i32>().map(|_v| Self::None),
            }
        } else if s.trim().len() == 0 {
            println!("none");
            Ok(Self::None)
        } else {
            println!("dot");
            let coords: Vec<&str> = s.trim().split(',').collect();

            let x_fromstr = coords[0].parse::<i32>()?;
            let y_fromstr = coords[1].parse::<i32>()?;

            Ok(Self::Point(x_fromstr, y_fromstr))
        }
    }
}
