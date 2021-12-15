use std::fs::File;
use std::io::{BufRead, BufReader};

use log::debug;
use log::info;

fn main() {
    env_logger::init();
    let filename = "input.d1p1.full";

    // Open the file in read-only mode (ignoring errors).

    let file = File::open(filename).unwrap();

    let reader = BufReader::new(file);

    let count = reader
        .lines()
        .filter_map(|v| v.map(|v| v.parse::<i32>().unwrap()).ok())
        .fold(None, accumulate)
        .map(|(_, count)| count)
        .unwrap_or(-1);
    info!("Count = {}", count);
}

fn accumulate(acc: Option<(i32, i32)>, reading: i32) -> Option<(i32, i32)> {
    debug!("reading: {}", reading);
    match acc {
        Some((last, count)) if reading > last => Some((reading, count + 1)),
        Some((_last, count)) => Some((reading, count)),
        None => Some((reading, 0)),
    }
}
