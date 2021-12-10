use std::fs::File;

use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input";

    // Open the file in read-only mode (ignoring errors).

    let file = File::open(filename).unwrap();

    let reader = BufReader::new(file);

    let mut position = (0, 0, 0);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut s = line.split(" ");
        let d = (s.next().unwrap(), s.next().unwrap().parse::<i32>().unwrap());
        println!("{:?}", d);
        match d {
            ("forward", v) => {
                position.0 += v;
                position.1 += v * position.2
            }
            ("down", v) => position.2 += v,
            ("up", v) => position.2 -= v,
            (o, _v) => panic!("Unexpected direction '{}'", o),
        }
    }
    println!("position = {:?}", position);
    println!("result: {}", position.0 * position.1);
}
