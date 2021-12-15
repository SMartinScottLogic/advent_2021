use std::fs::File;

use std::io::{BufRead, BufReader};

use log::{debug, info};
use std::collections::HashMap;

fn main() {
    env_logger::init();

    let filename = "input.d3p1.full";

    // Open the file in read-only mode (ignoring errors).

    let file = File::open(filename).unwrap();

    let reader = BufReader::new(file);

    let (count, freq) = reader
        .lines()
        .map(Result::unwrap)
        .fold((0, HashMap::new()), update_bit_counts);

    let (gamma, epsilon) = calculate_rates(count, &freq);
    info!("total = {} x {} = {}", epsilon, gamma, epsilon * gamma);
}

fn calculate_rates(count: i32, freq: &HashMap<usize, i32>) -> (i32, i32) {
    let mut epsilon = 0;
    let mut gamma = 0;
    for (k, reading) in freq {
        debug!("{} {} / {}", k, reading, count);
        let increment = 1 << k;
        if *reading > (count >> 1) {
            epsilon += increment;
        } else {
            gamma += increment;
        }
    }
    (gamma, epsilon)
}

fn update_bit_counts(
    (count, mut acc): (i32, HashMap<usize, i32>),
    v: String,
) -> (i32, HashMap<usize, i32>) {
    for (index, value) in v.chars().rev().enumerate() {
        if value == '1' {
            *acc.entry(index).or_insert(0) += 1;
        }
    }
    (count + 1, acc)
}
