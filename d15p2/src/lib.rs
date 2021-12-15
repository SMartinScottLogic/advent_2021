use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

use utils::Matrix;

pub fn load(filename: &str) -> Solution {
    let file = File::open(filename).unwrap();

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
                    println!("{} {} {}", nx, ny, nv);
                    self.data.set(nx, ny, nv);
                }
                }
            }
        }
        println!("({}, {})", self.xsize, self.xsize);
        self.xsize = ((self.xsize + 1) * xfactor as isize) - 1;
        self.ysize = ((self.ysize + 1) * yfactor as isize) - 1;
        println!("({}, {})", self.xsize, self.ysize);
    }

    fn display(&self) {
        for y in 0..=self.ysize {
            for x in 0..=self.xsize {
                print!("{}", self.data.get(x, y).unwrap_or(&-1));
            }
            println!();
        }
        println!();
    }

    fn next(&self, visited: &HashSet<(isize, isize)>, distance: &HashMap<(isize, isize), i64>) -> (isize, isize) {
        let mut next = (0, 0);
        let mut best_cost: i64 = -1;
        for ((x, y), cost) in distance {
            if visited.contains(&(*x, *y)) {
                continue;
            }
                if best_cost == -1 || *cost < best_cost {
                    best_cost = *cost;
                    next = (*x, *y);
                }
        }
        next
    }

    pub fn analyse(&mut self) {
        //return;
        let mut visited = HashSet::new();
        let mut distance = HashMap::new();
        distance.entry((0, 0)).or_insert(0);
        // loop here ?

        loop {
            let (x, y) = self.next(&visited, &distance);

        let cur = *distance.entry((x, y)).or_insert(0);
        // Up
        if let Some(value) = self.data.get(x, y - 1) {
            let cost = cur + value;
            let curcost = distance.get(&(x, y - 1)).unwrap_or(&-1);
            if *curcost == -1 || *curcost > cost {
                *distance.entry((x, y - 1)).or_insert(0) = cost;
            }
        }
        // Down
        if let Some(value) = self.data.get(x, y + 1) {
            let cost = cur + value;
            let curcost = distance.get(&(x, y + 1)).unwrap_or(&-1);
            if *curcost == -1 || *curcost > cost {
                *distance.entry((x, y + 1)).or_insert(0) = cost;
            }
        }
        // Left
        if let Some(value) = self.data.get(x - 1, y) {
            let cost = cur + value;
            let curcost = distance.get(&(x - 1, y)).unwrap_or(&-1);
            if *curcost == -1 || *curcost > cost {
                *distance.entry((x - 1, y)).or_insert(0) = cost;
            }
        }
        // Right
        if let Some(value) = self.data.get(x + 1, y) {
            let cost = cur + value;
            let curcost = distance.get(&(x + 1, y)).unwrap_or(&-1);
            if *curcost == -1 || *curcost > cost {
                *distance.entry((x + 1, y)).or_insert(0) = cost;
            }
        }

        visited.insert((x, y));
        // self.display(&distance);
        if x == self.xsize && y == self.xsize {
            println!("done");
            break;
        }
        println!("next: {:?} {} / {}", self.next(&visited, &distance), visited.len(), (self.xsize + 1) * (self.ysize + 1));
        //break;
    }
    println!("{}", self.xsize);
    println!("{}", self.ysize);
        self.answer = *distance.get(&(self.xsize, self.ysize)).unwrap_or(&-1);
    }

    pub fn answer(&self) -> i64 {
        self.answer as i64
    }
}
