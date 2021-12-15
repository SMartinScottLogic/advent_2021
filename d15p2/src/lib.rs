use log::debug;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::Matrix;

pub fn load(filename: &str) -> Solution {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(e) => {
            println!("Failed to open {} file error: {:?}", filename, e);
            panic!();
        }
    };

    let reader = BufReader::new(file);

    let mut solution = Solution::new();
    for (y, line) in reader.lines().enumerate() {
        for (x, value) in line.unwrap().trim().chars().enumerate() {
            solution.add(x, y, value.to_string().parse().unwrap());
        }
    }
    solution.display();
    solution.expand(5, 5);
    solution.display();
    solution
}

#[derive(Debug, Default)]
pub struct Solution {
    data: Matrix,
    xsize: isize,
    ysize: isize,
    answer: i64,
}

impl Solution {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn add(&mut self, x: usize, y: usize, value: i64) {
        self.data.set(x as isize, y as isize, value);
        self.xsize = max(x.try_into().unwrap(), self.xsize);
        self.ysize = max(y.try_into().unwrap(), self.ysize);
    }

    fn expand(&mut self, xfactor: usize, yfactor: usize) {
        let (sx, sy) = self.data.dimensions();
        for y in 0..=sy {
            for x in 0..=sx {
                let v = self.data.get(x, y).unwrap().to_owned();
                for dy in 0..yfactor {
                    for dx in 0..xfactor {
                        let mut nv = v + dx as i64 + dy as i64;
                        while nv > 9 {
                            nv -= 9;
                        }
                        let nx = x + (dx as isize * (1 + sx));
                        let ny = y + (dy as isize * (1 + sy));
                        debug!("{} {} {}", nx, ny, nv);
                        self.data.set(nx, ny, nv);
                    }
                }
            }
        }
        debug!("({}, {})", self.xsize, self.xsize);
        self.xsize = ((self.xsize + 1) * xfactor as isize) - 1;
        self.ysize = ((self.ysize + 1) * yfactor as isize) - 1;
        debug!("({}, {})", self.xsize, self.ysize);
    }

    fn display(&self) {
        for y in 0..=self.ysize {
            let mut row = "".to_string();
            for x in 0..=self.xsize {
                row.push_str(&format!("{}", self.data.get(x, y).unwrap_or(&-1)));
            }
            debug!("{}", row);
        }
    }

    fn next(&self, notvisited: &HashMap<(isize, isize), i64>) -> (isize, isize) {
        let mut next = (0, 0);
        let mut best_cost: i64 = -1;
        for ((x, y), cost) in notvisited {
            let cost = cost.to_owned();
            if best_cost == -1 || (cost != -1 && cost < best_cost) {
                best_cost = cost;
                next = (*x, *y);
            }
        }
        if best_cost < 0 {
            panic!();
        }
        next
    }

    pub fn analyse(&mut self) {
        //return;
        let mut notvisited = HashSet::new();
        {
            let (xsize, ysize) = self.data.dimensions();
            for y in 0..=ysize {
                for x in 0..=xsize {
                    notvisited.insert((x, y));
                }
            }
        }
        let mut distance = HashMap::new();
        let mut notvisited_scored = HashMap::new();

        distance.entry((0, 0)).or_insert(0);
        notvisited_scored.entry((0, 0)).or_insert(0);
        // loop here ?

        loop {
            let (x, y) = self.next(&notvisited_scored);

            let cur = *distance.entry((x, y)).or_insert(0);
            for sy in -1isize..=1 {
                for sx in -1isize..=1 {
                    if isize::abs(sx) == isize::abs(sy) {
                        continue;
                    }
                    if let Some(value) = self.data.get(x + sx, y + sy) {
                        let cost = cur + value;
                        if notvisited.contains(&(x + sx, y + sy)) {
                            let s = notvisited_scored.entry((x + sx, y + sy)).or_insert(-1);
                            if *s == -1 || *s > cost {
                                *s = cost;
                                *distance.entry((x + sx, y + sy)).or_insert(0) = cost;
                            }
                        }
                    }
                }
            }

            notvisited.remove(&(x, y));
            notvisited_scored.remove(&(x, y));
            // self.display(&distance);
            if x == self.xsize && y == self.xsize {
                // Done
                break;
            }
            debug!(
                "next: {:?} {} / {}",
                self.next(&notvisited_scored),
                notvisited.len(),
                (self.xsize + 1) * (self.ysize + 1)
            );
            //break;
        }
        debug!("{}", self.xsize);
        debug!("{}", self.ysize);
        self.answer = *distance.get(&(self.xsize, self.ysize)).unwrap_or(&-1);
    }

    pub fn answer(&self) -> i64 {
        self.answer as i64
    }
}
