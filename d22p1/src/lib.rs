use anyhow::{Context, Result};
use log::{debug, trace};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Error};
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename).with_context(|| format!("Failed to read from {}", filename))?;

    let reader = BufReader::new(file);

    let mut solution = Solution::new();
    for line in reader.lines() {
        let line = Line::from_str(line?.trim()).unwrap();
        solution.add(line);
    }
    Ok(solution)
}

#[derive(Debug, Default)]
pub struct Solution {
    answer: i64,
    input: Vec<Line>,
}

impl Solution {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn analyse(&mut self) {
        let mut status = HashMap::new();
        for line in &self.input {
            debug!("line: {:?}", line);
            for z in line.sz..=line.ez {
                for y in line.sy..=line.ey {
                    for x in line.sx..=line.ex {
                        trace!("({},{},{}) => {:?}", x, y, z, line.mode);
                        match line.mode {
                            Mode::On => *status.entry((x, y, z)).or_insert(1) = 1,
                            Mode::Off => *status.entry((x, y, z)).or_insert(0) = 0,
                        };
                    }
                }
            }
        }
        self.answer = 0;
        for (pos, v) in status {
            if pos.0 >= -50
                && pos.0 <= 50
                && pos.1 >= -50
                && pos.1 <= 50
                && pos.2 >= -50
                && pos.2 <= 50
            {
                self.answer += v;
            }
        }
    }

    pub fn answer(&self) -> Result<i64> {
        Ok(self.answer)
    }
}

impl Solution {
    fn clamp(mut value: i64) -> i64 {
        if value < -50 {
            value = -51;
        }
        if value > 50 {
            value = 51;
        }
        value
    }

    fn add(&mut self, mut line: Line) {
        // clamp to wanted range
        line.sx = Self::clamp(line.sx);
        line.ex = Self::clamp(line.ex);
        line.sy = Self::clamp(line.sy);
        line.ey = Self::clamp(line.ey);
        line.sz = Self::clamp(line.sz);
        line.ez = Self::clamp(line.ez);

        self.input.push(line);
    }
}

#[derive(Debug)]
enum Mode {
    On,
    Off,
}

impl FromStr for Mode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            _ => Err(std::io::Error::from(io::ErrorKind::InvalidInput)),
        }
    }
}

#[derive(Debug)]
struct Line {
    mode: Mode,
    sx: i64,
    ex: i64,
    sy: i64,
    ey: i64,
    sz: i64,
    ez: i64,
}

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        debug!("line: {}", s);
        let re =
            Regex::new(r"^(?P<mode>[^\s]+) x=(?P<sx>-?\d+)\.\.(?P<ex>-?\d+),y=(?P<sy>-?\d+)\.\.(?P<ey>-?\d+),z=(?P<sz>-?\d+)\.\.(?P<ez>-?\d+)$").unwrap();
        let capt = re.captures(s).unwrap();
        let mode = Mode::from_str(capt.name("mode").unwrap().as_str()).unwrap();
        let sx: i64 = capt.name("sx").unwrap().as_str().parse().unwrap();
        let ex: i64 = capt.name("ex").unwrap().as_str().parse().unwrap();
        let sy: i64 = capt.name("sy").unwrap().as_str().parse().unwrap();
        let ey: i64 = capt.name("ey").unwrap().as_str().parse().unwrap();
        let sz: i64 = capt.name("sz").unwrap().as_str().parse().unwrap();
        let ez: i64 = capt.name("ez").unwrap().as_str().parse().unwrap();

        Ok(Self {
            mode,
            sx,
            ex,
            sy,
            ey,
            sz,
            ez,
        })
    }
}
