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
        let l = line
            .unwrap()
            .split('|')
            .take(2)
            .map(|v| Line::from_str(v).unwrap())
            .collect::<Vec<_>>();

        println!("{:?} => {:?}", l[0], l[1]);
        solution.add(l.get(0).unwrap().to_owned(), l.get(1).unwrap().to_owned());
    }

    solution
}

#[derive(Debug)]
pub struct Solution {
    data: Vec<(Line, Line)>,
    answer: i64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Line {
    positions: Vec<Vec<char>>,
}

impl Solution {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            answer: 0i64,
        }
    }

    fn add(&mut self, signal: Line, output: Line) {
        self.data.push((signal, output));
    }

    pub fn analyse(&mut self) {
        self.answer = 0;
        for (input, output) in &self.data {
            let mut mapping = HashMap::new();
            let mut r_mapping = HashMap::new();
            for entry in input.entry() {
                let value = match entry.len() {
                    2 => 1,
                    3 => 7,
                    4 => 4,
                    5 => -1,
                    6 => -1,
                    7 => 8,
                    _ => panic!(),
                };
                if value >= 0 {
                    mapping.entry(entry.clone()).or_insert(value);
                    r_mapping.entry(value).or_insert_with(|| entry.clone());
                }
            }
            let one = r_mapping
                .get(&1)
                .unwrap()
                .iter()
                .map(|v| v.to_owned())
                .collect::<HashSet<_>>();
            let four = r_mapping
                .get(&4)
                .unwrap()
                .iter()
                .map(|v| v.to_owned())
                .collect::<HashSet<_>>();
            println!("mapping: {:?}", mapping);

            for entry in input.entry() {
                if !mapping.contains_key(entry) {
                    let this = entry.iter().map(|v| v.to_owned()).collect::<HashSet<_>>();
                    let value = match entry.len() {
                        6 => match (
                            this.intersection(&four).count(),
                            this.intersection(&one).count(),
                        ) {
                            (4, _) => 9,
                            (_, 2) => 0,
                            _ => 6,
                        },
                        5 => match (
                            this.intersection(&one).count(),
                            this.intersection(&four).count(),
                        ) {
                            (2, _) => 3,
                            (_, 2) => 2,
                            _ => 5,
                        },
                        _ => panic!(),
                    };
                    if value >= 0 {
                        mapping.entry(entry.clone()).or_insert(value);
                    }
                }
            }

            println!("mapping: {:?}", mapping);

            let mut line_score = 0;
            for entry in output.entry() {
                let digit = mapping.get(entry).unwrap();
                line_score *= 10;
                line_score += digit;
            }
            self.answer += line_score;
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
    fn entry(&self) -> impl Iterator<Item = &Vec<char>> + '_ {
        self.positions.iter()
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let positions = s
            .trim()
            .split_whitespace()
            .map(|v| v.chars().sorted().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Ok(Line { positions })
    }
}
