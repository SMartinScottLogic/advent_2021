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

    fn counterpoint(&self, c: &char) -> char {
        match c {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!(),
        }
    }

    pub fn analyse(&mut self) {
        self.answer = 0;
        let mut autocomplete_scores = Vec::new();
        for line in &self.data {
            let mut opened = Vec::new();
            let mut illegal = false;
            for (pos, x) in line.data.iter().enumerate() {
                illegal = match x {
                    '(' | '[' | '{' | '<' => {
                        opened.push(x);
                        false
                    }
                    _ => match opened.pop() {
                        None => {
                            true
                        }
                        Some(y) if !self.matched_pair(y, x) => {
                            true
                        }
                        _ => false,
                    },
                };
                if illegal {
                    println!("illegal {},{} in {:?}", pos, x, line);
                    //self.answer += score;
                    break;
                }
            }
            if illegal {
                continue;
            }
            println!("incomplete: {:?} {:?}", line, opened);
            let mut score = 0;
            loop {
                match opened.pop() {
                    None => break,
                    Some(c) => {
                        score *= 5;
                        let c = self.counterpoint(c);
                        score += match c {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            '>' => 4,
                            _ => panic!(),
                        };
                    }
                }
            }
            println!("line autocomplete score = {}", score);
            autocomplete_scores.push(score);
        }
        let answer_pos = autocomplete_scores.len() / 2;
        self.answer = *autocomplete_scores
            .iter()
            .sorted()
            .skip(answer_pos)
            .next()
            .unwrap();
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
