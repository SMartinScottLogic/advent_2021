use std::fs::File;

use std::io::{BufRead, BufReader};

fn sum(values: &[Result<String, std::io::Error>]) -> i32 {
    let mut total = 0;
    for v in values {
        if let Ok(value) = v {
        let reading: i32 = value.parse().unwrap();
        total += reading;
        }
    }
    total
}

fn main() {

    let filename = "input";

    // Open the file in read-only mode (ignoring errors).

    let file = File::open(filename).unwrap();

    let reader = BufReader::new(file);

    let mut last: Option<i32> = None;
    let mut count = 0;

    let values: Vec<_> = reader.lines().collect();
    let sums = values.windows(3).map(sum).collect::<Vec<_>>();
    println!("{:?}", sums);
    for sum in sums {
        match last {
            Some(v) if sum > v => count += 1,
            _ => {}
        }
        last = Some(sum);
    }
    /*

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let reading: i32 = line.parse().unwrap();

        match last {
            Some(v) if reading > v => count += 1,
            _ => {}
        }
        // Show the line and its number.

        println!("{}. {}", index + 1, reading);
        last = Some(reading);

    }
    */
    println!("Count = {}", count);

}
