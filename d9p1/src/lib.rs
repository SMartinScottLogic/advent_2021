use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

use itertools::Itertools;

pub fn load(filename: &str) -> Solution {
    let file = File::open(filename).unwrap();

    let reader = BufReader::new(file);

    let mut solution = Solution::new();
    for line in reader.lines() {
        solution.add(Line::from_str(&line.unwrap()).unwrap());
    }

    solution
}

#[derive(Debug)]
pub struct Solution {
    data: Vec<Line>,
    answer: i64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Line {
    positions: Vec<u8>,
}

impl Solution {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            answer: 0i64,
        }
    }

    fn add(&mut self, signal: Line) {
        self.data.push(signal);
    }

    pub fn analyse(&mut self) {
        self.answer = 0;
        for (y, xdata) in self.data.iter().enumerate() {
            for (x, height) in xdata.entry().enumerate() {
                //println!("({} {}) => {}", x, y, height);
                let mut lowest = true;
                // Above
                if y > 0 {
                    if let Some(h) = self.data.get(y - 1).and_then(|l| l.positions.get(x)) {
                        if h <= height {
                            lowest = false;
                        }
                    }
                }
                // Below
                if let Some(h) = self.data.get(y + 1).and_then(|l| l.positions.get(x)) {
                    if h <= height {
                        lowest = false;
                    }
                }
                // Left
                if x > 0 {
                    if let Some(h) = self.data.get(y).and_then(|l| l.positions.get(x - 1)) {
                        if h <= height {
                            lowest = false;
                        }
                    }
                }
                // Right
                if let Some(h) = self.data.get(y).and_then(|l| l.positions.get(x + 1)) {
                    if h <= height {
                        lowest = false;
                    }
                }
                if lowest {
                    self.answer += 1 + *height as i64;
                    println!("({}, {}) {}", x, y, height);
                }
            }
        }
        println!("{}", self.answer);
    }

    /*
    1 : [ ,  , c,  ,  , f,  ] = 2
    7 : [a,  , c,  ,  , f,  ] = 3
    4 : [ , b, c, d,  , f,  ] = 4

    2 : [a,  , c, d, e,  , g] = 5
    3 : [a,  , c, d,  , f, g] = 5
    5 : [a, b,  , d,  , f, g] = 5

    0 : [a, b, c,  , e, f, g] = 6
    6 : [a, b,  , d, e, f, g] = 6
    9 : [a, b, c, d,  , f, g] = 6

    8 : [a, b, c, d, e, f, g] = 7
        [8, 6, 8, 7, 4, 9, 7]
     */

    pub fn answer(&self) -> i64 {
        self.answer as i64
    }
}

impl Line {
    fn entry(&self) -> impl Iterator<Item = &u8> + '_ {
        self.positions.iter()
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let positions = s
            .trim()
            .chars()
            .map(|v| v.to_string())
            .map(|v| v.parse::<u8>().unwrap())
            .collect::<Vec<_>>();

        Ok(Line { positions })
    }
}
