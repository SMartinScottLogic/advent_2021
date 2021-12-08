use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

pub fn load(filename: &str) -> Solution {
    let file = File::open(filename).unwrap();

    let reader = BufReader::new(file);

    let mut solution = Solution::new();
    for line in reader.lines().take(1) {
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
    data: Vec<(Line, Line)>,
    answer: i64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Line {
    positions: Vec<HashSet<char>>,
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
     */
    pub fn analyse(&mut self) {
        self.answer = 0;
        for (input, output) in &self.data {
            let mut mapping: HashMap<_, _> = ('a'..='g')
                .map(|k| (k, ('a'..'g').collect::<Vec<_>>()))
                .collect();

            println!("mapping: {:?}", mapping);
            println!("input: {:?}", input);
            for single in input.entry() {
                let len = single.len();
                for digit in single {
                    match len {
                        2 => {
                            // 1 only digit with 2 segments: [c, f]
                            mapping
                                .get_mut(&digit)
                                .unwrap()
                                .retain(|v| *v == 'c' || *v == 'f');
                            println!("mapping: {:?}", mapping);
                        }
                        3 => {
                            // 7 only digit with 3 segments: [a, c, f]
                            mapping
                                .get_mut(&digit)
                                .unwrap()
                                .retain(|v| *v == 'a' || *v == 'c' || *v == 'f');
                        },
                        4 => {
                            // 4 only digit with 4 segments: [b, c, d, f]
                            mapping
                                .get_mut(&digit)
                                .unwrap()
                                .retain(|v| *v == 'b' || *v == 'c' || *v == 'd' || *v == 'f');
                        },
                        7 => {
                            // 8 only digit with 7 segments: [a, b, c, d, e, f, g]
                            // => retain all
                        },
                        _ => {}
                        }
                }
            }
            println!("{:?} => mapping: {:?}", (input, output), mapping);
        }
    }

    pub fn answer(&self) -> i64 {
        self.answer as i64
    }
}

impl Line {
    fn entry(&self) -> impl Iterator<Item = &HashSet<char>> + '_ {
        self.positions.iter()
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let positions = s
            .trim()
            .split_whitespace()
            .map(|v| v.chars().collect::<HashSet<_>>())
            .collect::<Vec<_>>();

        Ok(Line { positions })
    }
}
