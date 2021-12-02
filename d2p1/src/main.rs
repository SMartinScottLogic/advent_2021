use std::fs::File;

use std::io::{BufRead, BufReader};



fn main() {

    let filename = "input";

    // Open the file in read-only mode (ignoring errors).

    let file = File::open(filename).unwrap();

    let reader = BufReader::new(file);

    let mut last: Option<i32> = None;
    let mut count = 0;
    let mut position = (0, 0);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut s = line.split(" ");
        let d = (s.next().unwrap(), s.next().unwrap().parse::<i32>().unwrap());
        println!("{:?}", d);
        match d {
            ("forward", v) => position.0 += v,
            ("down", v) => position.1 += v,
            ("up", v) => position.1 -= v,
            (o, v) => panic!("Unexpected direction '{}'", o)
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
    println!("position = {:?}", position);
    println!("result: {}", position.0 * position.1);

}
