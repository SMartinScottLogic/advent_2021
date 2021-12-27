use anyhow::Context;
use log::{debug, info, trace};
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
    Ok(solution)
}

#[derive(Debug, Default)]
pub struct Solution {
    answer: Option<i64>,
    amphipods: String,

    tentative_costs: HashMap<String, i64>,
    has_visited: HashSet<String>,
}

impl Solution {
    fn new() -> Self {
        Self {
            amphipods: r".".repeat(19),
            ..Default::default()
        }
    }

    pub fn analyse(&mut self) {
        self.tentative_costs = HashMap::new();
        let mut node = self.amphipods.clone();
        let mut cost = 0;
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
                    info!("next: {:?} {} {}", n, c, Self::is_complete(&n));
                    node = n.to_owned();
                    cost = c.to_owned();
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
    }

    pub fn answer(&self) -> Option<i64> {
        self.answer
    }
}

impl Solution {
    fn add_amphipod(&mut self, amphipod: char, x: usize, y: usize) {
        //self.amphipods.push((Point::new(x, y), amphipod));
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
    fn is_complete(nodes: &str) -> bool {
        nodes == r"...........ABCDABCD"
    }

    fn to_world(idx: usize) -> (i64, i64) {
        let idx = idx as i64;
        if idx <= 10 {
            (idx + 1, 1)
        } else if idx <= 14 {
            (2 * (idx - 11) + 3, 2)
        } else if idx <= 18 {
            (2 * (idx - 15) + 3, 3)
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
        } else {
            panic!("Illegal coords: ({},{})", x, y);
        }
    }

    fn is_path(x: i64, y: i64) -> bool {
        (y == 1 && (1..=11).contains(&x))
            || ((y == 2 || y == 3) && (x == 3 || x == 5 || x == 7 || x == 9))
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
        for y in 1..=3 {
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
        let mut possible_moves = HashMap::new();
        let mut considered = HashSet::new();

        let mut root = Vec::new();

        for (idx, _) in origin.chars().enumerate().filter(|(_, c)| *c != '.') {
            considered.insert(origin.to_owned());

            let (x, y) = Self::to_world(idx);
            let a_type = origin.chars().nth(idx).unwrap();
            // Do not move if at bottom of correct room
            if y == 3
                && match a_type {
                    'A' if x == 3 => true,
                    'B' if x == 5 => true,
                    'C' if x == 7 => true,
                    'D' if x == 9 => true,
                    _ => false,
                }
            {
                trace!("Do not move {} at {},{}", a_type, x, y);
                continue;
            }
            // Do not move if at top of correct room, and same type at bottom
            if y == 2
                && match a_type {
                    'A' if x == 3 => true,
                    'B' if x == 5 => true,
                    'C' if x == 7 => true,
                    'D' if x == 9 => true,
                    _ => false,
                }
                && origin
                    .chars()
                    .nth(Self::from_world(x, y + 1))
                    .unwrap()
                    .eq(&a_type)
            {
                trace!("Do not move {} at {},{}", a_type, x, y);
                continue;
            }
            root.push((origin.to_owned(), idx, idx, 0_i64));
        }

        while let Some((current, start_idx, idx, cost)) = root.pop() {
            // Convert idx -> world coords
            let a_type = current.chars().nth(idx).unwrap();
            let (x, y) = Self::to_world(idx);
            let (_, sy) = Self::to_world(start_idx);
            // Do not move if at bottom of correct room
            if y == 3
                && match a_type {
                    'A' if x == 3 => true,
                    'B' if x == 5 => true,
                    'C' if x == 7 => true,
                    'D' if x == 9 => true,
                    _ => false,
                }
            {
                trace!("Do not move {} at {},{}", a_type, x, y);
                continue;
            }
            // Do not move if at top of correct room, and same type at bottom
            if y == 2
                && match a_type {
                    'A' if x == 3 => true,
                    'B' if x == 5 => true,
                    'C' if x == 7 => true,
                    'D' if x == 9 => true,
                    _ => false,
                }
                && current
                    .chars()
                    .nth(Self::from_world(x, y + 1))
                    .unwrap()
                    .eq(&a_type)
            {
                trace!("Do not move {} at {},{}", a_type, x, y);
                continue;
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
                    if ((y + dy) == 2 || (y + dy) == 3)
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
                    // Cannot move to top of room iff wrong lower
                    if ((y + dy) == 2)
                        && match a_type {
                            'A' if (x + dx) == 3 => true,
                            'B' if (x + dx) == 5 => true,
                            'C' if (x + dx) == 7 => true,
                            'D' if (x + dx) == 9 => true,
                            _ => false,
                        }
                        && next
                            .chars()
                            .nth(Self::from_world(x, 3))
                            .unwrap()
                            .ne(&a_type)
                    {
                        continue;
                    }
                    // Cannot move within hallway
                    if (y + dy) == 1 && sy == 1 {
                        continue;
                    }
                    // Cannot stop in doorway
                    if (y + dy) == 1
                        && ((x + dx) == 3 || (x + dx) == 5 || (x + dx) == 7 || (x + dx) == 9)
                    {
                        continue;
                    }
                    debug!("possible: {} to {}", current, next);
                    possible_moves
                        .entry(next.clone())
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
        Self::dump(origin, &0);
        for (m, cost) in &possible_moves {
            Self::dump(m, cost);
        }

        possible_moves
    }

    fn update_costs(&mut self, amphipods: &str, cost: i64) {
        let possible_moves = Self::possible_moves(amphipods);
        Self::dump(amphipods, &cost);
        for (target, c) in possible_moves {
            Self::dump(&target, &c);
            let tentative_cost = c + cost;
            debug!(
                "Can move from {:?}) to {:?} for {}",
                amphipods, target, tentative_cost
            );
            if self.has_visited.contains(&target) {
                continue;
            }
            self.tentative_costs
                .entry(target.to_owned())
                .and_modify(|v| *v = min(*v, tentative_cost))
                .or_insert(tentative_cost);
        }
    }

    fn next(&self) -> Option<(String, i64)> {
        let mut best = None;
        for (node, tentative_cost) in &self.tentative_costs {
            if !self.has_visited.contains(node) {
                match best {
                    Some((_, c)) if c <= *tentative_cost => {}
                    _ => {
                        best = Some((node.to_owned(), tentative_cost.to_owned()));
                    }
                };
            }
        }
        best
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
    fn regression() {
        let m = Solution::possible_moves(r"...B.......B.CDADCA");
        debug!("{:?} moves", m);
        assert_eq!(m.contains_key(r"...........BBCDADCA"), false);
    }
}
