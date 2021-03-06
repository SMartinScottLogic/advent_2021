use anyhow::{Context, Result};
use enum_iterator::IntoEnumIterator;
use log::{debug, trace};
use regex::Regex;
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::ops::Add;
use std::str::FromStr;

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename).with_context(|| format!("Failed to read from {}", filename))?;

    let reader = BufReader::new(file);

    let mut solution = Solution::new();
    let mut scanner = None;
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with("--- ") {
            if let Some(scanner) = scanner {
                solution = solution + scanner;
            }
            scanner = Some(Scanner::new(line.to_string()));
        } else {
            scanner = scanner.map(|s| s + ScannerLine::from_str(line).unwrap());
        }
    }
    if let Some(scanner) = scanner {
        solution = solution + scanner;
    }
    Ok(solution)
}

#[derive(Debug, Default)]
pub struct Solution {
    data: Vec<Scanner>,
    answer: i64,
}

impl Solution {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn analyse(&mut self) {
        while self.data.len() > 1 {
            debug!("data len: {}", self.data.len());

            let mut changed = false;
            let mut next_data = Vec::new();
            while let Some(mut a) = self.data.pop() {
                let mut inner_next_data = Vec::new();
                for b in &self.data {
                    let mut new_b = self.calculate_overlap(&a, b);
                    if new_b.is_fixed() {
                        debug!("move {}", new_b.data.len());
                        for line in new_b.data {
                            a.data.insert(line);
                            changed = true;
                        }
                        debug!("{} len {}", a.name, a.data.len());
                    } else {
                        debug!("retain {}", new_b.name);
                        new_b.set_fixed(false);
                        inner_next_data.push(new_b);
                    }
                }
                debug!("complete: {}", a.name);
                next_data.push(a);
                self.data = inner_next_data;
            }
            if !changed {
                panic!();
            }
            self.data = next_data;
            debug!("data len: {}", self.data.len());
        }
        self.answer = self.data.get(0).unwrap().data.len() as i64;
    }

    pub fn answer(&self) -> Result<i64> {
        Ok(self.answer)
    }
}

impl Solution {
    fn calculate_overlap(&self, a: &Scanner, b: &Scanner) -> Scanner {
        for facing in Facing::into_enum_iter() {
            for rotation in [0, 90, 180, 270] {
                let r = b.reorientate(&facing, rotation);
                for av in &a.data {
                    for rv in &r.data {
                        let dx = av.x - rv.x;
                        let dy = av.y - rv.y;
                        let dz = av.z - rv.z;
                        let mut tb = r.translate(dx, dy, dz);
                        let num_overlaps = tb.overlap(a);
                        if num_overlaps >= 12 {
                            debug!(
                                "{} overlaps {}: {} (posn: {}, {}, {})",
                                tb.name, a.name, num_overlaps, dx, dy, dz
                            );
                            tb.set_fixed(true);
                            for d in &tb.data {
                                debug!("{}", d);
                            }
                            return tb;
                        }
                    }
                }
            }
        }
        b.to_owned()
    }
}

impl Add<Scanner> for Solution {
    type Output = Self;

    fn add(mut self, other: Scanner) -> Self {
        self.data.push(other);
        self
    }
}

#[derive(Debug, Default, PartialEq)]
struct Scanner {
    name: String,
    data: HashSet<ScannerLine>,
    fixed: bool,
}

impl Scanner {
    fn new(line: String) -> Scanner {
        Scanner {
            name: line,
            fixed: false,
            ..Default::default()
        }
    }

    fn to_owned(&self) -> Self {
        self.data
            .iter()
            .fold(Self::new(self.name.clone()), |scanner, line| {
                scanner + line.to_owned()
            })
    }

    fn set_fixed(&mut self, fixed: bool) {
        self.fixed = fixed;
    }

    fn is_fixed(&self) -> bool {
        self.fixed
    }

    fn reorientate(&self, facing: &Facing, rotation: usize) -> Self {
        self.data
            .iter()
            .fold(Scanner::new(self.name.clone()), |reorientated, line| {
                reorientated + line.reorientate(facing, rotation)
            })
    }

    fn translate(&self, dx: i64, dy: i64, dz: i64) -> Self {
        self.data
            .iter()
            .fold(Scanner::new(self.name.clone()), |translated, line| {
                translated + line.translate(dx, dy, dz)
            })
    }

    fn overlap(&self, other: &Self) -> usize {
        trace!("---");
        self.data.intersection(&other.data).count()
    }
}

impl Add<ScannerLine> for Scanner {
    type Output = Self;

    fn add(mut self, other: ScannerLine) -> Self {
        self.data.insert(other);
        self
    }
}

#[derive(Debug, Default, PartialEq, Hash, Eq)]
struct ScannerLine {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(IntoEnumIterator)]
enum Facing {
    PX,
    PY,
    PZ,
    NX,
    NY,
    NZ,
}

impl ScannerLine {
    fn to_owned(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    fn reorientate(&self, facing: &Facing, rotation: usize) -> Self {
        use Facing::*;

        let (x, y, z) = (self.x, self.y, self.z);
        let (x, y, z) = match facing {
            PX => (x, y, z),
            PY => (-y, x, z),
            PZ => (-z, y, x),
            NX => (-x, y, -z),
            NY => (y, -x, z),
            NZ => (z, y, -x),
        };
        let (x, y, z) = match rotation {
            0 => (x, y, z),
            90 => (x, -z, y),
            180 => (x, -y, -z),
            270 => (x, z, y),
            _ => unreachable!(),
        };
        Self { x, y, z }
    }

    fn translate(&self, dx: i64, dy: i64, dz: i64) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
            z: self.z + dz,
        }
    }
}
impl FromStr for ScannerLine {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(?P<x>-?\d+),(?P<y>-?\d+),(?P<z>-?\d+)").unwrap();
        let capt = re.captures(s).unwrap();
        let x = capt.name("x").unwrap().as_str().parse().unwrap();
        let y = capt.name("y").unwrap().as_str().parse().unwrap();
        let z = capt.name("z").unwrap().as_str().parse().unwrap();

        Ok(Self { x, y, z })
    }
}
impl fmt::Display for ScannerLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use crate::Facing::*;
    use crate::*;
    use std::str::FromStr;

    #[ctor::ctor]
    fn init() {
        env_logger::init();
    }

    fn make_scanner(input: Vec<&str>) -> Scanner {
        input
            .iter()
            .fold(Scanner::new("test".to_string()), |scanner, v| {
                let scanner = scanner + ScannerLine::from_str(v).unwrap();
                scanner
            })
    }

    #[test]
    fn line_orientate() {
        let line = ScannerLine::from_str(r"1,2,3").unwrap();
        assert_eq!(line.reorientate(&PX, 0).to_string(), r"1,2,3");
        assert_eq!(line.reorientate(&PY, 0).to_string(), r"-2,1,3");
    }

    fn has_orientation(a: &Scanner, b: &Scanner) -> bool {
        for facing in Facing::into_enum_iter() {
            for rotation in [0, 90, 180, 270] {
                let b2 = b.reorientate(&facing, rotation);
                debug!("{:?}", b2);
                if *a == b2 {
                    return true;
                }
            }
        }
        false
    }

    #[test]
    fn orientate() {
        let scanner1 = make_scanner(vec![
            r"-1,-1,1", r"-2,-2,2", r"-3,-3,3", r"-2,-3,1", r"5,6,-4", r"8,0,7",
        ]);
        let scanner2 = make_scanner(vec![
            r"-1,-1,1", r"-2,-2,2", r"-3,-3,3", r"-2,-3,1", r"5,6,-4", r"8,0,7",
        ]);
        assert!(has_orientation(&scanner1, &scanner2));

        let scanner2 = make_scanner(vec![
            r"-1,-1,-1",
            r"-2,-2,-2",
            r"-3,-3,-3",
            r"-1,-3,-2",
            r"4,6,5",
            r"-7,0,8",
        ]);
        assert!(has_orientation(&scanner1, &scanner2));

        let scanner2 = make_scanner(vec![
            r"1,1,-1", r"2,2,-2", r"3,3,-3", r"1,3,-2", r"-4,-6,5", r"7,0,8",
        ]);
        assert!(has_orientation(&scanner1, &scanner2));

        let scanner2 = make_scanner(vec![
            r"1,1,1",
            r"2,2,2",
            r"3,3,3",
            r"3,1,2",
            r"-6,-4,-5",
            r"0,7,-8",
        ]);
        assert!(has_orientation(&scanner1, &scanner2));
    }
}
