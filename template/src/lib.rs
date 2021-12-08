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

    Solution::new()
}

#[derive(Debug)]
pub struct Solution {    
}


impl Solution {
    fn new() -> Self {
        Self {}
    }

    pub fn analyse(&mut self) {
    }

    pub fn answer(&self) -> i64 {
        0
    }
}