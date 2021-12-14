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
    template: String,
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
            Line::Template(template) => self.template = template,
            Line::Rule(s, t) => self.rules.push((s, t)),
            _ => {}
        }
    }

    pub fn analyse(&mut self) {
        let mut template = self.template.clone();
        for _i in 1..=10 {
            let mut next = String::new();
            for p in 0..template.len() - 1 {
                for (s, t) in &self.rules {
                    let key = &template[p..p + 2];
                    if key == s {
                        next.push(key.chars().next().unwrap());
                        next.push_str(&t);
                    }
                }
            }
            next.push_str(&template[template.len() - 1..]);
            template = next;
        }
        println!("template: {}", template);
        let freq = template.chars().fold(HashMap::new(), |mut acc, v| {
            *acc.entry(v).or_insert(0) += 1;
            acc
        });
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
        println!("{} {}", min, max);
        self.answer = max - min;
    }

    pub fn answer(&self) -> i64 {
        self.answer as i64
    }
}

enum Line {
    Template(String),
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
        } else if s.trim().len() == 0 {
            Ok(Self::None)
        } else {
            let template = s.trim().to_string();
            Ok(Self::Template(template))
        }
    }
}
