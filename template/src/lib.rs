use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename)?;

    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line)?;

    Ok(Solution::new())
}

#[derive(Debug)]
pub struct Solution {}

impl Solution {
    fn new() -> Self {
        Self {}
    }

    pub fn analyse(&mut self) {}

    pub fn answer(&self) -> Result<i64> {
        Ok(0)
    }
}
