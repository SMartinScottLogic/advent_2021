use std::fs::File;
use std::num::ParseIntError;
use std::str::FromStr;
use std::cmp::{min, max};
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub fn load(filename: &str) -> Solution {
    let file = File::open(filename).unwrap();

    let reader = BufReader::new(file);

    let mut solution = Solution::new();
    for line in reader.lines() {
        let line = line.unwrap().to_owned();
        let segment = LineSegment::from_str(&line).unwrap();
        solution.add(segment);
    }
    solution
}
#[derive(Debug)]
pub struct Solution {
    line_segments: Vec<LineSegment>,
    scores: HashMap<(i32, i32), i32>
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct LineSegment {
    start: Point,
    end: Point,
}

impl Solution {
    fn new() -> Self {
        Self {
            line_segments: Vec::new(),
            scores: HashMap::new(),
        }
    }

    pub fn analyse(&mut self) {
        for line_segment in &self.line_segments {
            println!("{:?}", line_segment);

            if line_segment.is_horizontal() {
                let start_x = min(line_segment.start.x, line_segment.end.x);
                let start_y = min(line_segment.start.y, line_segment.end.y);
                let end_x = max(line_segment.start.x, line_segment.end.x);
                let end_y = max(line_segment.start.y, line_segment.end.y);
                let y = end_y;
                for x in start_x..=end_x {
                    println!("({}, {})", x, y);
                    *self.scores.entry((x, y)).or_insert(0) += 1;
                }                
            } else if line_segment.is_vertical() {
                let start_x = min(line_segment.start.x, line_segment.end.x);
                let start_y = min(line_segment.start.y, line_segment.end.y);
                let end_x = max(line_segment.start.x, line_segment.end.x);
                let end_y = max(line_segment.start.y, line_segment.end.y);
                let x = end_x;
                for y in start_y..=end_y {
                    println!("({}, {})", x, y);
                    *self.scores.entry((x, y)).or_insert(0) += 1;
                }                
            } else {
                let reverse = line_segment.start.x > line_segment.end.x;
                let start_x = if reverse {line_segment.end.x } else { line_segment.start.x };
                let start_y = if reverse {line_segment.end.y } else { line_segment.start.y };
                let end_x = if reverse { line_segment.start.x } else { line_segment.end.x };
                let end_y = if reverse { line_segment.start.y } else { line_segment.end.y };
                println!("({}, {}) -> ({}, {})", start_x, start_y, end_x, end_y);
                let mut delta = 1;
                if end_y < start_y {
                    delta = -1;
                }
                let mut y = start_y;
                for x in start_x..=end_x {
                    println!("({}, {})", x, y);
                    *self.scores.entry((x, y)).or_insert(0) += 1;
                    y += delta;
                }
            }
        }
        println!("{:?}", self.scores);
    }

    pub fn answer(&self) -> i64 {
        let mut total = 0;
        for (_pos, score) in &self.scores {
            if *score > 1 {
                total += 1;
            }
        }
        total
    }
}

impl LineSegment {
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Point {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}

impl FromStr for LineSegment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(" -> ").map(Point::from_str).filter_map(|v| v.ok());
        let start = iter.next().unwrap();
        let end = iter.next().unwrap();
        Ok(LineSegment { start, end })
    }
}

impl Solution {
    fn add(&mut self, line_segment: LineSegment) {
        self.line_segments.push(line_segment);
    }
}
