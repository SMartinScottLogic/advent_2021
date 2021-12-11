use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::Matrix;

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

#[derive(Debug, Default)]
pub struct Solution {
    data: Matrix,
    answer: i64,
}

impl Solution {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn add(&mut self, x: isize, y: isize, value: i64) {
        self.data.set(x, y, value)
    }

    fn get(&self, x: isize, y: isize) -> Option<i64> {
        self.data.get(x, y).and_then(|v| Some(v.to_owned()))
    }

    fn flash(&mut self) {
        let (xsize, ysize) = self.data.dimensions();

        let mut flashed = HashSet::new();
        let mut changed = true;
        while changed {
            changed = false;
            for y in 0..=ysize {
                for x in 0..=xsize {
                    if let Some(score) = self.data.get(x, y) {
                        if *score > 9 {
                            if !flashed.contains(&(x, y)) {
                                changed = true;
                                flashed.insert((x, y));
                                for ystep in -1..=1 {
                                    for xstep in -1..=1 {
                                        match self
                                            .get(x + xstep, y + ystep)
                                            .and_then(|v| Some(v + 1))
                                        {
                                            Some(value) => self.add(x + xstep, y + ystep, value),
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn display(&self) {
        let (xsize, ysize) = self.data.dimensions();
        for y in 0..=ysize {
            for x in 0..=xsize {
                if let Some(score) = self.data.get(x, y) {
                    print!("{} ", score);
                }
            }
            println!();
        }
    }

    pub fn analyse(&mut self) {
        self.answer = 0;
        let (xsize, ysize) = self.data.dimensions();

        for step in 1..=100 {
            println!("step {}", step);
            // Increment all energy
            for y in 0..=ysize {
                for x in 0..=xsize {
                    let value = self.data.get(x, y).unwrap() + 1;
                    self.data.set(x, y, value);
                }
            }
            // Flashes
            self.flash();
            // Count flashes + Zero
            for y in 0..=ysize {
                for x in 0..=xsize {
                    if let Some(score) = self.data.get(x, y) {
                        if *score > 9 {
                            self.answer += 1;
                            self.data.set(x, y, 0);
                        }
                    }
                }
            }
            self.display();
        }
        println!("{}", self.answer);
    }

    pub fn answer(&self) -> i64 {
        self.answer as i64
    }
}
