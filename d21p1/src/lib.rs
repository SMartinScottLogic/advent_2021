use anyhow::{Context, Result};
use log::debug;
use regex::Regex;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename).with_context(|| format!("Failed to read from {}", filename))?;

    let reader = BufReader::new(file);

    let mut solution = Solution::new();
    for line in reader.lines() {
        let player = Player::from_str(line?.trim()).unwrap();
        solution.add(player);
    }

    Ok(solution)
}

#[derive(Debug, Default)]
pub struct Solution {
    players: Vec<Player>,
    answer: i64,
}

impl Solution {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn analyse(&mut self) {
        let mut dice: DeterministicDice = Default::default();
        self.answer = 0;
        let mut rolls = 0;

        let winning_id = loop {
            let mut winning_id = None;
            for player in &mut self.players {
                rolls += 3;
                let roll = (dice.roll(), dice.roll(), dice.roll());
                debug!("roll: {:?}", roll);
                let roll = roll.0 + roll.1 + roll.2;
                let mut position = player.position + roll;
                while position > 10 {
                    position -= 10;
                }
                player.score += position;
                player.position = position;
                debug!("player: {:?}", player);

                if player.score >= 1000 {
                    winning_id = Some(player.id);
                    break;
                }
            }
            if let Some(id) = winning_id {
                break id;
            }
        };
        let losing_score = self
            .players
            .iter()
            .filter(|player| player.id != winning_id)
            .map(|player| player.score)
            .next()
            .unwrap();
        debug!("losing_score: {}", losing_score);
        debug!("rolls: {}", rolls);
        self.answer = losing_score * rolls;
    }

    pub fn answer(&self) -> Result<i64> {
        Ok(self.answer)
    }
}

impl Solution {
    fn add(&mut self, player: Player) {
        self.players.push(player);
    }
}

trait Dice {
    fn roll(&mut self) -> i64;
}

#[derive(Default)]
struct DeterministicDice {
    current: i64,
}

impl Dice for DeterministicDice {
    fn roll(&mut self) -> i64 {
        self.current += 1;
        if self.current > 100 {
            self.current = 1;
        }
        self.current
    }
}

#[derive(Debug, Default)]
struct Player {
    id: i64,
    position: i64,
    score: i64,
}

impl Player {}

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
