use anyhow::{Context, Result};
use log::{debug, trace};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename).with_context(|| format!("Failed to read from {}", filename))?;

    let reader = BufReader::new(file);

    let mut solution = Solution::new();
    let mut players = HashMap::new();
    for line in reader.lines() {
        let player = Player::from_str(line?.trim()).unwrap();
        players.entry(player.id).or_insert(player);
    }
    debug!("players: {:?}", players);
    let world = World::new(
        players[&1].position,
        players[&2].position,
        players[&1].score,
        players[&1].score,
    );
    solution.worlds.entry(world).or_insert(1);
    Ok(solution)
}

#[derive(Debug, Default)]
pub struct Solution {
    worlds: HashMap<World, i64>,
    wins: HashMap<i64, i64>,
    answer: i64,
    max_score: i64,
}

#[derive(Debug, Default, PartialEq, Eq, Hash)]
struct World {
    p1_position: i64,
    p2_position: i64,
    p1_score: i64,
    p2_score: i64,
}

impl World {
    fn new(p1_position: i64, p2_position: i64, p1_score: i64, p2_score: i64) -> World {
        World {
            p1_position,
            p2_position,
            p1_score,
            p2_score,
        }
    }
}

impl Solution {
    fn new() -> Self {
        Self {
            max_score: 21,
            ..Default::default()
        }
    }

    pub fn analyse(&mut self) {
        while !self.worlds.is_empty() {
            //for _pass in 1..=1 {
            self.step();
            debug!("wins: {:?}", self.wins);
            trace!("worlds: {:?}", self.worlds);
            debug!("#worlds: {}", self.worlds.len());
        }
        self.answer = 0;
    }

    pub fn answer(&self) -> Result<i64> {
        Ok(self.answer)
    }
}

impl Solution {
    fn step(&mut self) {
        let initial_worlds = self.worlds.len();
        let mut new_worlds = HashMap::new();
        for (world, count) in &self.worlds {
            trace!("world: {:?}", world);
            let mut new_positions = HashMap::new();
            for (id, position, score) in [
                (1, world.p1_position, world.p1_score),
                (2, world.p2_position, world.p2_score),
            ] {
                for roll1 in 1..=3 {
                    for roll2 in 1..=3 {
                        for roll3 in 1..=3 {
                            let roll = roll1 + roll2 + roll3;
                            let mut new_position = position + roll;
                            while new_position > 10 {
                                new_position -= 10;
                            }
                            let new_score = score + new_position;
                            new_positions
                                .entry(id)
                                .or_insert_with(Vec::new)
                                .push((new_position, new_score));
                        }
                    }
                }
            }
            trace!("{:?}", new_positions[&1]);
            for (id1, player_positions1) in &new_positions {
                for (id2, player_positions2) in &new_positions {
                    if id2 <= id1 {
                        continue;
                    }
                    for (player1_position, player1_score) in player_positions1 {
                        if player1_score >= &self.max_score {
                            *self.wins.entry(1).or_insert(0) += count;
                            trace!("p1 wins: ({}, {})", player1_position, player1_score);
                        } else {
                            for (player2_position, player2_score) in player_positions2 {
                                trace!(
                                    "({}, {}) vs ({}, {})",
                                    player1_position,
                                    player1_score,
                                    player2_position,
                                    player2_score
                                );
                                if player2_score >= &self.max_score {
                                    *self.wins.entry(2).or_insert(0) += count;
                                    trace!(
                                        "p2 wins: ({}, {}) vs ({}, {})",
                                        player1_position,
                                        player1_score,
                                        player2_position,
                                        player2_score
                                    );
                                } else {
                                    let world = World::new(
                                        *player1_position,
                                        *player1_score,
                                        *player2_position,
                                        *player2_score,
                                    );
                                    trace!("{:?} {}", world, count);
                                    *new_worlds.entry(world).or_insert(0) += count;
                                }
                            }
                        }
                    }
                }
            }
            trace!("{:?}", self.wins);
        }
        debug!("step {} -> {}", initial_worlds, new_worlds.len());
        for world in &new_worlds {
            trace!("   {:?}", world);
        }
        self.worlds = new_worlds;
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash)]
struct Player {
    id: i64,
    position: i64,
    score: i64,
}

impl FromStr for Player {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re =
            Regex::new(r"^Player (?P<player>\d+) starting position: (?P<position>\d+)$").unwrap();
        let capt = re.captures(s).unwrap();
        let player = capt.name("player").unwrap().as_str().parse().unwrap();
        let position = capt.name("position").unwrap().as_str().parse().unwrap();

        Ok(Self {
            id: player,
            position,
            ..Default::default()
        })
    }
}
