use anyhow::{Context, Result};
use log::{debug, info};
use regex::Regex;
use std::arch::x86_64::_mm256_hsubs_epi16;
use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename).with_context(|| format!("Failed to read from {}", filename))?;

    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line)?;

    Solution::from_str(&line)
        .with_context(|| format!("Failed to parse solution input from {}", line))
}

#[derive(Debug, Default)]
pub struct Solution {
    sx: i64,
    ex: i64,
    sy: i64,
    ey: i64,
    answer: i64,
}

impl Solution {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn analyse(&mut self) {
        let mut max_height = 0;
        let mut num_hit = 0;
        for vy in -300..=300 {
            for vx in 0..=max(self.sx, self.ex) {
                match self.simulate(vx, vy) {
                    true => {
                        let height = self.answer;
                        debug!("hit ({} {}) => {} vs {}", vx, vy, height, max_height);
                        max_height = max(max_height, height);
                        num_hit += 1;
                    }
                    false => {
                        //debug!("miss ({} {}) => {}", vx, vy, self.answer);
                    }
                };
            }
        }
        self.answer = num_hit;
    }

    pub fn answer(&self) -> Result<i64> {
        Ok(self.answer)
    }

    fn in_target_area(&self, x: i64, y: i64) -> bool {
        x >= self.sx && x <= self.ex && y >= self.sy && y <= self.ey
    }

    fn simulate(&mut self, mut vx: i64, mut vy: i64) -> bool {
        let mut x = 0;
        let mut y = 0;
        self.answer = 0;

        let hit = loop {
            x += vx;
            y += vy;
            self.answer = max(self.answer, y);
            vx += match vx {
                _p if vx > 0 => -1,
                _n if vx < 0 => 1,
                _z => 0,
            };
            vy -= 1;

            if self.in_target_area(x, y) {
                break true;
            }
            if y < min(self.sy, self.ey) {
                break false;
            }
        };

        hit
    }
}

impl FromStr for Solution {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"^target area: x=(?P<sx>-?\d+)\.\.(?P<ex>-?\d+), y=(?P<sy>-?\d+)\.\.(?P<ey>-?\d+)$",
        )
        .unwrap();
        debug!("input: {}", s);
        let caps = re.captures(s.trim()).unwrap();
        for name in re.capture_names() {
            match name {
                Some(n) => debug!("{} => {:?}", n, caps.name(n).map(|v| v.as_str())),
                None => {}
            };
        }
        let sx = caps.name("sx").unwrap().as_str().parse::<i64>().unwrap();
        let sy = caps.name("sy").unwrap().as_str().parse::<i64>().unwrap();
        let ex = caps.name("ex").unwrap().as_str().parse::<i64>().unwrap();
        let ey = caps.name("ey").unwrap().as_str().parse::<i64>().unwrap();

        Ok(Self {
            sx,
            ex,
            sy,
            ey,
            answer: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use std::str::FromStr;

    #[ctor::ctor]
    fn init() {
        env_logger::init();
    }

    #[test]
    fn parse_input() {
        let solution = Solution::from_str("target area: x=20..30, y=-10..-5").unwrap();
        assert_eq!(solution.sx, 20);
        assert_eq!(solution.ex, 30);
        assert_eq!(solution.sy, -10);
        assert_eq!(solution.ey, -5);
    }

    #[test]
    fn in_target_area() {
        let solution = Solution::from_str("target area: x=20..30, y=-10..-5").unwrap();
        assert_eq!(solution.in_target_area(20, -10), true);
        assert_eq!(solution.in_target_area(21, -11), false);
        assert_eq!(solution.in_target_area(30, -5), true);
        assert_eq!(solution.in_target_area(20, -4), false);
    }

    #[test]
    fn hit() {
        let mut solution = Solution::from_str("target area: x=20..30, y=-10..-5").unwrap();
        assert_eq!(solution.simulate(7, 2), true);
        assert_eq!(solution.simulate(6, 3), true);
        assert_eq!(solution.simulate(9, 0), true);
        assert_eq!(solution.simulate(17, -4), false);
    }

    #[test]
    fn height() {
        let mut solution = Solution::from_str("target area: x=20..30, y=-10..-5").unwrap();
        assert_eq!(solution.simulate(6, 9), true);
        assert_eq!(solution.answer, 45);
    }

    #[test]
    fn hit_count() {
        let mut solution = Solution::from_str("target area: x=20..30, y=-10..-5").unwrap();
        solution.analyse();
        assert_eq!(solution.answer().unwrap(), 112);
    }
}
