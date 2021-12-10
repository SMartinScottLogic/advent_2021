use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

pub fn load(filename: &str) -> Solution {
    let file = File::open(filename).unwrap();

    let reader = BufReader::new(file);

    let mut solution = Solution::new();
    for line in reader.lines() {
        let l = line
            .unwrap()
            .split("|")
            .take(2)
            .map(|v| Line::from_str(v).unwrap().to_owned())
            .collect::<Vec<_>>();

        println!("{:?} => {:?}", l[0], l[1]);
        solution.add(l.get(0).unwrap().to_owned(), l.get(1).unwrap().to_owned());
    }

    solution
}

#[derive(Debug)]
pub struct Solution {
    data: HashMap<Line, Line>,
    answer: i64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Line {
    positions: Vec<String>,
}

impl Solution {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
            answer: 0i64,
        }
    }

    fn add(&mut self, signal: Line, output: Line) {
        self.data.insert(signal, output);
    }

    pub fn analyse(&mut self) {
        self.answer = 0;
        for (_input, output) in &self.data {
            for single in output.entry() {
                self.answer += match single.len() {
                    2 => 1, // 1 only digit with 2 segments
                    3 => 1, // 7 only digit with 3 segments
                    4 => 1, // 4 only digit with 4 segments
                    7 => 1, // 8 only digit with 7 segments
                    _ => 0,
                }
            }
        }
    }

    pub fn answer(&self) -> i64 {
        self.answer as i64
    }
}

impl Line {
    fn entry(&self) -> impl Iterator<Item = &String> + '_ {
        self.positions.iter()
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let positions = s
            .trim()
            .split_whitespace()
            .map(|v| v.to_string())
            .collect::<Vec<_>>();

        Ok(Line { positions })
    }
}
