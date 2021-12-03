use std::fs::File;

use std::io::{BufRead, BufReader};

use std::collections::HashMap;

fn main() {

    let filename = "input";

    // Open the file in read-only mode (ignoring errors).

    let file = File::open(filename).unwrap();

    let reader = BufReader::new(file);

    let mut last: Option<i32> = None;
    let mut count = 0;
    let mut counts = HashMap::new();

    for line in reader.lines() {
        count += 1;
        let line = line.unwrap();
        let mut reading = u32::from_str_radix(&line, 2).unwrap();

        println!("{} -> {}", line, reading);
        let mut index = 0;
        while reading > 0 {
            let field = reading & 1;
            println!("  {} {}", index, field);
            *counts.entry(index).or_insert(0) += field;
            reading >>= 1;
            index += 1;
        }

        /*
        let reading: i32 = line.parse().unwrap();

        match last {
            Some(v) if reading > v => count += 1,
            _ => {}
        }
        // Show the line and its number.

        println!("{}. {}", index + 1, reading);
        last = Some(reading);
        */

    }
    println!("Count = {} {:?}", count, counts);

    let mut epsilon = 0;
    let mut gamma = 0;
    for (k, v) in counts {
        if v > (count >> 1) {
          let value = 1 << k;
          epsilon += value;
        } else {
          let value = 1 << k;
          gamma += value;
        }

    }
    println!("total = {} x {} = {}", epsilon, gamma, epsilon * gamma);
}
