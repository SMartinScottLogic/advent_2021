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
    population: Vec<i32>
}

impl FromStr for Solution {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let timers = s.split(',').map(|v| v.parse::<i32>().unwrap()).collect();

        Ok(Solution { population: timers })
    }
}

impl Solution {
    pub fn analyse(&mut self) {
        for days in 1..=80 {
            let mut next_population = Vec::new();
            let mut new = 0;
            for timer in &self.population {
                let mut next_timer = timer - 1;
                if *timer == 0 {
                    next_timer = 6;
                    new += 1;
                }
                next_population.push(next_timer);
            }
            for _i in 0..new {
                next_population.push(8);
            }
            self.population = next_population;
            println!("{} {:?}", days, self.population);
        }
    }

    pub fn answer(&self) -> i64 {
        self.population.len() as i64
    }
}
