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
    population: HashMap<i32, i64>
}

impl FromStr for Solution {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let timers = s.split(',').map(|v| v.parse::<i32>().unwrap())
        .fold(HashMap::new(), |mut acc, v| {
            let counter = acc.entry(v).or_insert(0i64);
            *counter += 1;
            acc    
        });

        Ok(Solution { population: timers })
    }
}

impl Solution {
    pub fn analyse(&mut self) {
        for days in 1..=12 {
            let mut new = 0;
            let mut next_population: HashMap<i32, i64> = self.population.iter().map(|(timer, count)| {
                match timer {
                    0 => (6, count.to_owned()),
                    v => (v-1, count.to_owned())
                }
            }).collect();
            *next_population.entry(8).or_insert(0) = next_population.get(&6).unwrap_or(&0).to_owned();
            self.population = next_population;
            println!("{} {:?}", days, self.answer());
        }
    }

    pub fn answer(&self) -> i64 {
        self.population.iter()
        .map(|v| {println!("{:?}", v); v})
        .fold(0i64, |mut acc, (_timer, count)| {acc += *count; acc})
    }
}
