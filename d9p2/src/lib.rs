use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use utils::Matrix;

use itertools::Itertools;

pub fn load(filename: &str) -> Solution {
    let file = File::open(filename).unwrap();

    let reader = BufReader::new(file);

    let mut solution = Solution::new();
    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (x, value) in line
            .trim()
            .chars()
            .map(|v| v.to_string())
            .map(|v| v.parse::<i64>().unwrap())
            .enumerate()
        {
            solution.add(x.try_into().unwrap(), y.try_into().unwrap(), value);
        }
    }

    solution
}

#[derive(Debug)]
pub struct Solution {
    data: Matrix,
    answer: i64,
}

impl Solution {
    fn new() -> Self {
        Self {
            data: Matrix::new(),
            answer: 0i64,
        }
    }

    fn add(&mut self, x: isize, y: isize, value: i64) {
        self.data.set(x, y, value)
    }

    fn get(&self, x: isize, y: isize) -> Option<i64> {
        self.data.get(x, y).map(|v| v.to_owned())
    }

    pub fn analyse(&mut self) {
        self.answer = 0;
        let mut seed = Vec::new();
        let (xsize, ysize) = self.data.dimensions();
        for y in 0..=ysize {
            for x in 0..=xsize {
                let height = self.data.get(x, y).unwrap();
                //println!("({} {}) => {}", x, y, height);
                let mut lowest = true;
                // Above
                if let Some(h) = self.get(x, y - 1) {
                    if h <= *height {
                        lowest = false;
                    }
                }

                // Below
                if let Some(h) = self.get(x, y + 1) {
                    if h <= *height {
                        lowest = false;
                    }
                }
                // Left
                if let Some(h) = self.get(x - 1, y) {
                    if h <= *height {
                        lowest = false;
                    }
                }

                // Right
                if let Some(h) = self.get(x + 1, y) {
                    if h <= *height {
                        lowest = false;
                    }
                }
                if lowest {
                    seed.push((x, y));
                }
            }
        }
        println!("seed {:?}", seed);
        let mut sizes = Vec::new();
        for (x, y) in seed {
            let mut points = HashSet::new();
            points.insert((x, y));
            loop {
                let mut new_points = HashSet::new();
                for (x, y) in &points {
                    new_points.insert((*x, *y));
                    // Up
                    if self.get(*x, y - 1).unwrap_or(9) != 9 {
                        //println!("grow to ({}, {})", x, y - 1);
                        new_points.insert((*x, y - 1));
                    }
                    // Down
                    if self.get(*x, y + 1).unwrap_or(9) != 9 {
                        //println!("grow to ({}, {})", x, y + 1);
                        new_points.insert((*x, y + 1));
                    }
                    // Left
                    if self.get(x - 1, *y).unwrap_or(9) != 9 {
                        //println!("grow to ({}, {})", x-1, y);
                        new_points.insert((x - 1, *y));
                    }
                    // Right
                    if self.get(x + 1, *y).unwrap_or(9) != 9 {
                        //println!("grow to ({}, {})", x+1, y);
                        new_points.insert((x + 1, *y));
                    }
                }
                if points.len() < new_points.len() {
                    points = new_points;
                } else {
                    break;
                }
            }
            println!("({},{}) points {} = {:?}", x, y, points.len(), points);
            sizes.push(points.len());
        }
        self.answer = sizes
            .iter()
            .sorted()
            .rev()
            .take(3)
            .fold(1, |acc, v| acc * *v as i64);
        println!("{}", self.answer);
    }

    pub fn answer(&self) -> i64 {
        self.answer as i64
    }
}
