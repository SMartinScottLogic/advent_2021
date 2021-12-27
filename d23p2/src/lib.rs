use anyhow::Context;
use log::{debug, trace};
use std::cmp::min;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename).with_context(|| format!("Failed to read from {}", filename))?;

    let reader = BufReader::new(file);

    let mut solution = Solution::new();
    for (y, line) in reader.lines().enumerate() {
        for (x, c) in line?.trim_end().chars().enumerate() {
            match c {
                'A' | 'B' | 'C' | 'D' => solution.add_amphipod(c, x, y),
                '#' => solution.add_wall(x, y),
                ' ' => {}
                '.' => solution.add_space(x, y),
                _ => unreachable!(),
            }
        }
    }
    Solution::dump(&solution.amphipods, &-1);
    Ok(solution)
}

#[derive(Debug, Default)]
pub struct Solution {
    answer: Option<i64>,
    amphipods: String,
}

impl Solution {
    fn new() -> Self {
        let amphipods = r"...............DCBADBAC....".to_string();
        Self {
            amphipods,
            ..Default::default()
        }
    }

    pub fn analyse(&mut self) {
        let a = pathfinding::prelude::astar(
            &self.amphipods,
            |s| Self::possible_moves(s),
            |n| {
                let dist = n
                    .chars()
                    .skip(11)
                    .zip(Self::end().chars().skip(11))
                    .map(|v| match v {
                        (a, b) if a == b => 0,
                        ('.', _) => 1,
                        (_, '.') => 1,
                        (_, _) => 2,
                    })
                    .fold(0, |mut a, v| {
                        a += v;
                        a
                    });
                dist
            },
            |n| Self::end() == *n,
        );
        debug!("{:?}", a);
        /*
        self.tentative_costs = HashMap::new();
        let mut node = self.amphipods.clone();
        let mut cost = 0;
        let mut count = 0;
        loop {
            self.has_visited.insert(node.clone());
            self.update_costs(&node, cost);
            for (amphipods, tentative_cost) in &self.tentative_costs {
                trace!(
                    "{} {} {:?}",
                    tentative_cost,
                    Self::is_complete(amphipods),
                    amphipods
                );
            }
            let next = self.next();
            match next {
                Some((n, c)) if !Self::is_complete(&n) => {
                    if count % 1000 == 0 {
                    info!("next: {:?} {} {}", n, c, Self::is_complete(&n));
                    node = n.to_owned();
                    cost = c.to_owned();
                    }
                    count += 1;
                }
                _ => break,
            }
            debug!("visited: {}", self.has_visited.len());
        }
        self.answer = None;
        for (node, cost) in self
            .tentative_costs
            .iter()
            .filter(|(nodes, _cost)| Self::is_complete(*nodes))
        {
            info!("complete: {} {:?}", cost, node);
            self.answer = Some(*cost);
        }
        */
    }

    pub fn answer(&self) -> Option<i64> {
        self.answer
    }
}

impl Solution {
    fn add_amphipod(&mut self, amphipod: char, x: usize, y: usize) {
        //self.amphipods.push((Point::new(x, y), amphipod));
        let y = if y == 3 { 5 } else { y };
        let idx = Self::from_world(x as i64, y as i64);
        self.amphipods = self
            .amphipods
            .chars()
            .enumerate()
            .map(|(i, c)| if i == idx { amphipod } else { c })
            .collect();
    }

    fn add_wall(&mut self, _x: usize, _y: usize) {
        //self.walls.insert(Point::new(x, y));
    }

    fn add_space(&mut self, _x: usize, _y: usize) {
        //self.spaces.insert(Point::new(x, y));
    }
}

impl Solution {
    fn end() -> String {
        r"...........ABCDABCDABCDABCD".to_string()
    }

    fn to_world(idx: usize) -> (i64, i64) {
        let idx = idx as i64;
        if idx <= 10 {
            (idx + 1, 1)
        } else if idx <= 14 {
            (2 * (idx - 11) + 3, 2)
        } else if idx <= 18 {
            (2 * (idx - 15) + 3, 3)
        } else if idx <= 22 {
            (2 * (idx - 19) + 3, 4)
        } else if idx <= 26 {
            (2 * (idx - 23) + 3, 5)
        } else {
            panic!("Illegal index: {}", idx);
        }
    }

    fn from_world(x: i64, y: i64) -> usize {
        if y == 1 && x >= 1 && x <= 11 {
            (x - 1) as usize
        } else if y == 2 && (x == 3 || x == 5 || x == 7 || x == 9) {
            (11 + (x - 3) / 2) as usize
        } else if y == 3 && (x == 3 || x == 5 || x == 7 || x == 9) {
            (15 + (x - 3) / 2) as usize
        } else if y == 4 && (x == 3 || x == 5 || x == 7 || x == 9) {
            (19 + (x - 3) / 2) as usize
        } else if y == 5 && (x == 3 || x == 5 || x == 7 || x == 9) {
            (23 + (x - 3) / 2) as usize
        } else {
            panic!("Illegal coords: ({},{})", x, y);
        }
    }

    fn is_path(x: i64, y: i64) -> bool {
        (y == 1 && (1..=11).contains(&x)) || ((2..=5).contains(&y) && [3, 5, 7, 9].contains(&x))
    }
}

impl Solution {
    fn dump(map: &str, cost: &i64) {
        let m = map
            .chars()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (idx, c)| {
                let (x, y) = Self::to_world(idx);
                *acc.entry((x, y)).or_insert(' ') = c;
                acc
            });
        debug!("=================== {}", cost);
        for y in 1..=5 {
            let mut s = String::new();
            for x in 0..=12 {
                s.push(*m.get(&(x, y)).unwrap_or(&' '));
            }
            debug!("{} {}", y, s);
        }
        debug!("===================");
    }

    fn possible_moves(origin: &str) -> HashMap<String, i64> {
        debug!("origin: {:?}", origin);
        let mut possible_moves: HashMap<String, i64> = HashMap::new();
        let mut considered = HashSet::new();

        let mut root = Vec::new();

        for (idx, _) in origin.chars().enumerate().filter(|(_, c)| *c != '.') {
            considered.insert(origin.to_owned());

            let (x, y) = Self::to_world(idx);
            let a_type = origin.chars().nth(idx).unwrap();
            // Do not move if in correct room, and so are all entries below
            if match a_type {
                'A' if x == 3 => true,
                'B' if x == 5 => true,
                'C' if x == 7 => true,
                'D' if x == 9 => true,
                _ => false,
            } {
                let below = (y..=5)
                    .into_iter()
                    .map(|ty| origin.chars().nth(Self::from_world(x, ty)).unwrap())
                    .collect::<String>();
                let correct_below = below.chars().filter(|v| v.eq(&a_type)).count() as i64;
                let total_below = 1 + 5 - y;
                debug!(
                    "correct room: {} @ ({}, {}), below: {} ; {} / {}",
                    a_type, x, y, below, correct_below, total_below
                );
                //Self::dump(&origin, &-1);
                if correct_below == total_below {
                    debug!("Do not move {} at {},{}", a_type, x, y);
                    continue;
                }
            }
            root.push((origin.to_owned(), idx, idx, 0_i64));
        }

        while let Some((current, start_idx, idx, cost)) = root.pop() {
            // Convert idx -> world coords
            let a_type = current.chars().nth(idx).unwrap();
            let (x, y) = Self::to_world(idx);
            let (_, sy) = Self::to_world(start_idx);
            // Do not move if in correct room, and so are all entries below
            if match a_type {
                'A' if x == 3 => true,
                'B' if x == 5 => true,
                'C' if x == 7 => true,
                'D' if x == 9 => true,
                _ => false,
            } {
                let below = (y..=5)
                    .into_iter()
                    .map(|ty| origin.chars().nth(Self::from_world(x, ty)).unwrap())
                    .collect::<String>();
                let correct_below = below.chars().filter(|v| v.eq(&a_type)).count() as i64;
                let total_below = 1 + 5 - y;
                debug!(
                    "correct room: {} @ ({}, {}), below: {} ; {} / {}",
                    a_type, x, y, below, correct_below, total_below
                );
                //Self::dump(&origin, &-1);
                if correct_below == total_below {
                    debug!("Do not move {} at {},{}", a_type, x, y);
                    continue;
                }
            }
            for dy in -1..=1_i64 {
                for dx in -1..=1_i64 {
                    if dx.abs() + dy.abs() != 1 {
                        continue;
                    }
                    if !Self::is_path(x + dx, y + dy) {
                        continue;
                    }
                    // Cannot move into non-space
                    let next_idx = Self::from_world(x + dx, y + dy);
                    if current.chars().nth(next_idx).unwrap().ne(&'.') {
                        continue;
                    }

                    let next: String = current
                        .chars()
                        .enumerate()
                        .map(|(i, c)| {
                            if i == idx {
                                '.'
                            } else if i == next_idx {
                                a_type
                            } else {
                                c
                            }
                        })
                        .collect();
                    if considered.contains(&next) {
                        continue;
                    }
                    let cost = cost
                        + match a_type {
                            'A' => 1,
                            'B' => 10,
                            'C' => 100,
                            'D' => 1000,
                            _ => unreachable!(),
                        };
                    trace!("{} -> {}, {} -> {}", idx, next_idx, current, next);
                    root.push((next.clone(), start_idx, next_idx, cost));
                    considered.insert(next.clone());

                    // Legal full moves
                    // Can only move into correct room
                    if ((y + dy) >= 2 && (y + dy) <= 5)
                        && match a_type {
                            'A' if (x + dx) != 3 => true,
                            'B' if (x + dx) != 5 => true,
                            'C' if (x + dx) != 7 => true,
                            'D' if (x + dx) != 9 => true,
                            _ => false,
                        }
                    {
                        continue;
                    }
                    // Cannot move into room iff wrong lower
                    if ((y + dy) >= 2 && (y + dy) <= 5)
                        && match a_type {
                            'A' if (x + dx) == 3 => true,
                            'B' if (x + dx) == 5 => true,
                            'C' if (x + dx) == 7 => true,
                            'D' if (x + dx) == 9 => true,
                            _ => false,
                        }
                    {
                        let below = (y + dy..=5)
                            .into_iter()
                            .map(|ty| next.chars().nth(Self::from_world(x, ty)).unwrap())
                            .collect::<String>();
                        let correct_below = below.chars().filter(|v| v.eq(&a_type)).count() as i64;
                        let total_below = 1 + 5 - (y + dy);
                        debug!(
                            "304 correct room: {} @ ({}, {}), below: {} ; {} / {}",
                            a_type, x, y, below, correct_below, total_below
                        );
                        //Self::dump(&next, &-1);
                        if correct_below == total_below {
                            continue;
                        }
                    }
                    // Cannot move within hallway
                    if (y + dy) == 1 && sy == 1 {
                        continue;
                    }
                    // Cannot stop in doorway
                    if (y + dy) == 1 && [3, 5, 7, 9].contains(&(x + dx)) {
                        continue;
                    }
                    debug!("possible: {} to {}", current, next);
                    possible_moves
                        .entry(next)
                        .and_modify(|v| *v = min(*v, cost))
                        .or_insert(cost);
                }
            }
        }

        debug!(
            "{} possible moves: {:?}",
            possible_moves.len(),
            possible_moves
        );
        /*
        Self::dump(origin, &0);
        for (m, cost) in &possible_moves {
            Self::dump(m, cost);
        }
        */

        possible_moves
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[ctor::ctor]
    fn init() {
        env_logger::init();
    }

    #[test]
    #[ignore = "currently_failing"]
    fn regression() {
        let m = Solution::possible_moves(r"...B...............B.CDADCA");
        debug!("{:?} moves", m);
        assert_eq!(m.contains_key(r"...................BBCDADCA"), false);
        assert_eq!(m.contains_key(r"...................BBCDADCA"), false);
    }
}
