use std::fs::File;
use std::num::ParseIntError;
use std::str::FromStr;
use std::cmp::{min, max};
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub fn load(filename: &str) -> Solution {
    let file = File::open(filename).unwrap();

    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    Solution::from_str(&line).unwrap()
}

#[derive(Debug)]
pub struct Solution {    
    population: HashMap<i64, i64>
}

impl FromStr for Solution {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let positions = s.split(',').map(|v| v.parse::<i64>().unwrap())
        .fold(HashMap::new(), |mut acc, v| {
            let counter = acc.entry(v).or_insert(0i64);
            *counter += 1;
            acc    
        });

        Ok(Solution { population: positions })
    }
}

impl Solution {
    pub fn analyse(&mut self) {
        let mut best = -1i64;
        let mut best_target = 0;
        for (target_position, _count) in &self.population {
            let mut total = 0i64;
            for (position, count) in &self.population {
                total += (position - target_position).abs() * count;
            }
            if best < 0i64 || best > total {
                best = total;
                best_target = target_position.to_owned();
            }
        }
        println!("{} {}", best, best_target);
    }

    pub fn answer(&self) -> i64 {
        0
    }
}
