use std::fs::File;

use std::io::{BufRead, BufReader};

use log::{debug, info};

fn main() {
    env_logger::init();

    let filename = "input.d3p1.full";

    // Open the file in read-only mode (ignoring errors).

    let file = File::open(filename).unwrap();

    let reader = BufReader::new(file);

    let data: Vec<String> = reader.lines().map(|v| v.unwrap()).collect();
    println!("{:?}", data);

    let oxygen = calculate_rating(&data, |count_ones, total| {
        count_ones >= (total - count_ones)
    });
    info!("oxygen = {}", oxygen);
    let co2 = calculate_rating(&data, |count_ones, total| count_ones < (total - count_ones));
    info!("co2 = {}", co2);
    info!("{} x {} = {}", oxygen, co2, oxygen * co2);
}

fn calculate_rating(data: &[String], rule: impl Fn(usize, usize) -> bool) -> i64 {
    let mut needle = "".to_string();
    let mut last_match = data.len();
    loop {
        let mut probe = needle.clone();
        probe.push('1');
        let mut num_match = data.iter().filter(|v| (*v).starts_with(&probe)).count();
        if rule(num_match, last_match) {
            needle.push('1');
        } else {
            needle.push('0');
            num_match = last_match - num_match;
        }
        debug!(
            "probe {}=>{}, num_match {} vs {}",
            probe,
            needle,
            num_match,
            last_match - num_match
        );
        match data.iter().filter(|v| (*v).starts_with(&needle)).count() {
            1 => {
                return data
                    .iter()
                    .filter(|v| (*v).starts_with(&needle))
                    .fold(0, |_acc, v| {
                        debug!("{}", v);
                        i64::from_str_radix(v, 2).unwrap()
                    })
            }
            0 => panic!(),
            _ => {}
        };

        last_match = num_match;
    }
}
