use anyhow::Result;
use log::debug;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load(filename: &str) -> Result<Solution> {
    let file = File::open(filename)?;

    let mut solution = Solution::new();
    let reader = BufReader::new(file);

    for (y, line) in reader.lines().enumerate() {
        for (x, c) in line?.chars().enumerate() {
            solution.add(c, x, y);
        }
    }

    Ok(solution)
}

#[derive(Debug)]
pub struct Solution {
    answer: Option<i64>,
    east_cucumbers: HashSet<(usize, usize)>,
    south_cucumbers: HashSet<(usize, usize)>,

    maxx: usize,
    maxy: usize,
    world: HashMap<(usize, usize), Cucumber>,
}

#[derive(Debug)]
enum Cucumber {
    None,
    East,
    South,
}

impl Solution {
    fn new() -> Self {
        Self {
            answer: None,
            east_cucumbers: HashSet::new(),
            south_cucumbers: HashSet::new(),
            maxx: 0,
            maxy: 0,
            world: HashMap::new(),
        }
    }

    fn add(&mut self, c: char, x: usize, y: usize) {
        match c {
            '>' => self.east_cucumbers.insert((x, y)),
            'v' => self.south_cucumbers.insert((x, y)),
            _ => false,
        };
        self.maxx = max(self.maxx, x);
        self.maxy = max(self.maxy, y);
    }

    pub fn analyse(&mut self) {
        self.answer = None;
        self.update_world();

        self.dump();
        let mut num_passes = 0;
        loop
        {
            num_passes += 1;
            let last_changes = self.pass();
            debug!("last_changes: {}", last_changes);
            if last_changes == 0 {
                break;
            }
            self.dump();
        }
        self.answer = Some(num_passes);
    }

    pub fn answer(&self) -> Option<i64> {
        self.answer
    }
}

impl Solution {
    fn dump(&self) {
        for y in 0..=self.maxy {
            let mut line = String::new();
            for x in 0..=self.maxx {
                let c = if let Some(c) = self.world.get(&(x, y)) {
                    match *c {
                        Cucumber::East => '>',
                        Cucumber::South => 'v',
                        Cucumber::None => '.',
                    }
                } else {
                    '.'
                };
                line.push(c);
            }
            debug!("{}", line);
        }
    }

    fn pass(&mut self) -> i32 {
        let mut num_changes = 0;
        num_changes += Self::sub_pass(
            &mut self.east_cucumbers,
            self.maxx,
            self.maxy,
            &self.world,
            1,
            0,
        );
        self.update_world();
        num_changes += Self::sub_pass(
            &mut self.south_cucumbers,
            self.maxx,
            self.maxy,
            &self.world,
            0,
            1,
        );
        self.update_world();
        num_changes
    }

    fn sub_pass(
        cucumbers: &mut HashSet<(usize, usize)>,
        max_x: usize,
        max_y: usize,
        world: &HashMap<(usize, usize), Cucumber>,
        dx: usize,
        dy: usize,
    ) -> i32 {
        let mut num_changes = 0;

        let mut next_cucumbers = HashSet::new();
        for (x, y) in cucumbers.iter() {
            let mut next_x = x + dx;
            if next_x > max_x {
                next_x = 0;
            }
            let mut next_y = y + dy;
            if next_y > max_y {
                next_y = 0;
            }
            if let Some(Cucumber::None) = world.get(&(next_x, next_y)) {
                num_changes += 1;
                next_cucumbers.insert((next_x, next_y));
            } else {
                next_cucumbers.insert((*x, *y));
            }
        }
        cucumbers.drain();
        for (x, y) in next_cucumbers {
            cucumbers.insert((x, y));
        }
        num_changes
    }

    fn update_world(&mut self) {
        for y in 0..=self.maxy {
            for x in 0..=self.maxx {
                self.world.insert((x, y), Cucumber::None);
            }
        }
        for (x, y) in &self.east_cucumbers {
            self.world.insert((*x, *y), Cucumber::East);
        }
        for (x, y) in &self.south_cucumbers {
            self.world.insert((*x, *y), Cucumber::South);
        }
    }
}
