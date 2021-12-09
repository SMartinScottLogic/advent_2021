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

    fn get(&self, x: i64, y: i64) -> Option<u8> {
        if x < 0 || y < 0 {
            None
        } else {
            self.data
                .get(y as usize)
                .and_then(|l| l.positions.get(x as usize))
                .and_then(|v| Some(v.to_owned()))
        }
    }

    pub fn analyse(&mut self) {
        self.answer = 0;
        let mut seed = Vec::new();
        for (y, xdata) in self.data.iter().enumerate() {
            for (x, height) in xdata.entry().enumerate() {
                let x = x as i64;
                let y = y as i64;
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
                    if self.get(*x, y-1).unwrap_or(9) != 9 {
                            //println!("grow to ({}, {})", x, y - 1);
                            new_points.insert((*x, y-1));
                    }
                    // Down
                    if self.get(*x, y + 1).unwrap_or(9) != 9 {
                            //println!("grow to ({}, {})", x, y + 1);
                            new_points.insert((*x, y+1));
                    }
                    // Left
                    if self.get(x - 1, *y).unwrap_or(9) != 9 {
                            //println!("grow to ({}, {})", x-1, y);
                            new_points.insert((x-1, *y));
                    }
                    // Right
                    if self.get(x + 1, *y).unwrap_or(9) != 9 {
                            //println!("grow to ({}, {})", x+1, y);
                            new_points.insert((x+1, *y));
                        
                    }
                }
                if points.len() < new_points.len() {
                    points = new_points;
                } else  {
                    break;
                }
            }
            println!("({},{}) points {} = {:?}", x, y, points.len(),  points);
            sizes.push(points.len());
        }
        self.answer = sizes.iter().sorted().rev().take(3).fold(1, |acc, v| acc * *v as i64);
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
