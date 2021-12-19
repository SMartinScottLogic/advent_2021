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
        let line = line.unwrap();
        let line = Line::from_str(&line);
        solution.add(line.unwrap());
    }

    solution
}

#[derive(Debug, Default)]
pub struct Solution {
    answer: i64,
    template: HashMap<String, i64>,
    first: String,
    last: String,
    rules: Vec<(String, String)>,
}

impl Solution {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn add(&mut self, marks: Line) {
        match marks {
            Line::Template(first, last, template) => {
                self.first = first;
                self.last = last;
                self.template = template
            }
            Line::Rule(s, t) => self.rules.push((s, t)),
            _ => {}
        }
    }

    fn pairs(s: &str, count: i64) -> HashMap<String, i64> {
        let mut result = HashMap::new();
        for start in 0..s.len() - 1 {
            *result
                .entry(s[start..=start + 1].to_string())
                .or_insert(0i64) += count;
        }
        result
    }

    pub fn analyse(&mut self) {
        let mut template = self.template.clone();
        for _i in 1..=40 {
            template = template
                .into_iter()
                .map(|(k, v)| match self.rules.iter().find(|(s, _t)| *s == k) {
                    Some((s, t)) => {
                        let mut next = String::new();
                        let mut s = s.chars();
                        next.push(s.next().unwrap());
                        next.push_str(t);
                        next.push(s.next().unwrap());
                        Solution::pairs(&next, v)
                    }
                    _ => {
                        let mut next = HashMap::new();
                        next.insert(k, v);
                        next
                    }
                })
                .fold(HashMap::new(), |mut acc, m| {
                    for (k, v) in m {
                        *acc.entry(k).or_insert(0i64) += v;
                    }
                    acc
                });
        }
        println!("template: {:?}", template);
        let mut freq = template.iter().fold(HashMap::new(), |mut acc, (s, count)| {
            for c in s.chars() {
                *acc.entry(c).or_insert(0i64) += count;
            }
            acc
        });
        for c in self.first.chars() {
            *freq.entry(c).or_insert(0i64) += 1;
        }
        for c in self.last.chars() {
            *freq.entry(c).or_insert(0i64) += 1;
        }
        println!("freq {:?}", freq);
        let (min, max) = freq.into_iter().fold((-1, 1), |mut acc, (_k, v)| {
            if acc.0 == -1 || acc.0 > v {
                acc.0 = v;
            }
            if acc.1 == -1 || acc.1 < v {
                acc.1 = v;
            }
            acc
        });
        let max = max / 2;
        let min = min / 2;
        println!("{} {}", min, max);
        self.answer = max - min;
    }

    pub fn answer(&self) -> i64 {
        self.answer as i64
    }
}

enum Line {
    Template(String, String, HashMap<String, i64>),
    Rule(String, String),
    None,
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("->") {
            let mut rule = s.split("->").map(|v| v.trim());
            let source = rule.next().unwrap().to_string();
            let target = rule.next().unwrap().to_string();
            Ok(Self::Rule(source, target))
        } else if s.trim().is_empty() {
            Ok(Self::None)
        } else {
            let template = s.trim().to_string();
            let first = template[0..0].to_string();
            let last = template[template.len() - 1..].to_string();
            let template = template
                .chars()
                .collect::<Vec<char>>()
                .windows(2)
                .map(|c| {
                    println!("{:?}", c);
                    c
                })
                .map(|c| c.iter().collect::<String>())
                .fold(HashMap::new(), |mut acc, v| {
                    *acc.entry(v).or_insert(0i64) += 1;
                    acc
                });
            Ok(Self::Template(first, last, template))
        }
    }
}
