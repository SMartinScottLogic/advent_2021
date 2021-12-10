use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;
use utils::Matrix;

use itertools::Itertools;

pub fn load(filename: &str) -> Solution {
    let file = File::open(filename).unwrap();

    let reader = BufReader::new(file);

    let mut solution = Solution::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let line = Line::from_str(&line).unwrap();
        solution.add(line);
    }

    solution
}

#[derive(Debug)]
struct Line {
    data: Vec<char>,
}

#[derive(Debug, Default)]
pub struct Solution {
    data: Vec<Line>,
    answer: i64,
}

impl Solution {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn add(&mut self, line: Line) {
        self.data.push(line);
    }

    fn matched_pair(&self, a: &char, b: &char) -> bool {
        match (a, b) {
            ('(', ')') => true,
            ('[', ']') => true,
            ('{', '}') => true,
            ('<', '>') => true,
            _ => false,
        }
    }

    fn calculate_score(&self, c: &char) -> i64 {
        match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!(),
        }
    }

    pub fn analyse(&mut self) {
        self.answer = 0;
        for line in &self.data {
            let mut opened = Vec::new();
            for x in &line.data {
                let mut score = 0;
                let illegal = match x {
                    '(' | '[' | '{' | '<' => {
                        opened.push(x);
                        false
                    }
                    _ => match opened.pop() {
                        None => {
                            score += self.calculate_score(x);
                            true
                        }
                        Some(y) if !self.matched_pair(y, x) => {
                            score += self.calculate_score(x);
                            true
                        }
                        _ => false,
                    },
                };
                if illegal {
                    self.answer += score;
                    break;
                }
            }
        }
        println!("{}", self.answer);
    }

    pub fn answer(&self) -> i64 {
        self.answer as i64
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s.trim().chars().collect();

        Ok(Line { data })
    }
}
