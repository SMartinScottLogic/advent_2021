use anyhow::{Context, Result};
use log::debug;
use regex::Regex;
use std::cmp::{max, min};
use std::fs::File;
use std::io::{self, Error};
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename).with_context(|| format!("Failed to read from {}", filename))?;

    let reader = BufReader::new(file);

    let mut solution = Solution::new();
    for line in reader.lines() {
        let cube = Cube::from_str(line?.trim()).unwrap();
        solution.add(cube);
    }
    Ok(solution)
}

#[derive(Debug, Default)]
pub struct Solution {
    answer: i64,
    input: Vec<Cube>,
}

impl Solution {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn analyse(&mut self) {
        let mut cubes = Vec::new();
        for cube in &self.input {
            debug!("cube: {:?}", cube);
            let mut next_cubes = Vec::new();
            for c in &cubes {
                if let Some(mut overlap) = cube.overlap(c) {
                    debug!("overlap: {:?} between {:?} and {:?}", overlap, cube, c);
                    let overlap_mode = match c.mode {
                        Mode::On => Mode::Off,
                        Mode::Off => Mode::On,
                        _ => unreachable!(),
                    };
                    overlap.mode = overlap_mode;
                    next_cubes.push(overlap);
                }
            }
            if cube.mode == Mode::On {
                next_cubes.push(*cube);
            }
            cubes.append(&mut next_cubes);
        }
        self.answer = 0;
        for cube in cubes {
            let sign = match cube.mode {
                Mode::On => 1,
                Mode::Off => -1,
                _ => unreachable!(),
            };
            self.answer +=
                sign * (cube.ex - cube.sx + 1) * (cube.ey - cube.sy + 1) * (cube.ez - cube.sz + 1);
        }
    }

    pub fn answer(&self) -> Result<i64> {
        Ok(self.answer)
    }
}

impl Solution {
    fn add(&mut self, line: Cube) {
        self.input.push(line);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Mode {
    On,
    Off,
    None,
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

#[derive(Debug, Copy, Clone)]
struct Cube {
    mode: Mode,
    sx: i64,
    ex: i64,
    sy: i64,
    ey: i64,
    sz: i64,
    ez: i64,
}

impl Cube {
    fn overlap(&self, other: &Self) -> Option<Self> {
        if self.sx > other.ex
            || self.ex < other.sx
            || self.sy > other.ey
            || self.ey < other.sy
            || self.sz > other.ez
            || self.ez < other.sz
        {
            None
        } else {
            let sx = max(self.sx, other.sx);
            let ex = min(self.ex, other.ex);
            let sy = max(self.sy, other.sy);
            let ey = min(self.ey, other.ey);
            let sz = max(self.sz, other.sz);
            let ez = min(self.ez, other.ez);
            Some(Self {
                mode: Mode::None,
                sx,
                ex,
                sy,
                ey,
                sz,
                ez,
            })
        }
    }
}

impl FromStr for Cube {
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
            sx: min(sx, ex),
            ex: max(sx, ex),
            sy: min(sy, ey),
            ey: max(sy, ey),
            sz: min(sz, ez),
            ez: max(sz, ez),
        })
    }
}
