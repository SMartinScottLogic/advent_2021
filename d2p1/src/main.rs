use core::num::ParseIntError;
use core::str::FromStr;
use log::{debug, info};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    env_logger::init();
    let filename = "input.d2p1.full";

    // Open the file in read-only mode (ignoring errors).

    let file = File::open(filename).unwrap();

    let reader = BufReader::new(file);
    let position = reader
        .lines()
        .map(Result::unwrap)
        .filter_map(|v| Instruction::from_str(&v).ok())
        .fold((0, 0), accumulate);
    debug!("position = {:?}", position);
    info!("result: {}", position.0 * position.1);
}

fn accumulate(position: (i32, i32), instruction: Instruction) -> (i32, i32) {
    use Instruction::*;
    debug!("{:?}", instruction);
    match instruction {
        Forward(v) => (position.0 + v, position.1),
        Down(v) => (position.0, position.1 + v),
        Up(v) => (position.0, position.1 - v),
        _ => position,
    }
}

#[derive(Debug)]
enum Instruction {
    Forward(i32),
    Down(i32),
    Up(i32),
    None,
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().split(" ").take(2).collect::<Vec<_>>();
        let s = match &s[0..=1] {
            ["forward", v] => Self::Forward(v.parse().unwrap()),
            ["down", v] => Self::Down(v.parse().unwrap()),
            ["up", v] => Self::Up(v.parse().unwrap()),
            _ => Self::None,
        };
        Ok(s)
    }
}
