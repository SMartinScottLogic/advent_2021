use core::num;
use log::{debug, info};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

pub fn load(filename: &str) -> Solution {
    let file = File::open(filename).unwrap();

    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    let mut solution = Solution::new();
    solution.set_input(line);
    solution
}

#[derive(Default, Debug)]
pub struct Solution {
    input: String,
    answer: i64,
}

impl Solution {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn set_input(&mut self, input: String) {
        self.input = input;
    }

    fn bitstream(&self) -> String {
        self.input
            .chars()
            .flat_map(|c| {
                match c {
                    '0' => "0000",
                    '1' => "0001",
                    '2' => "0010",
                    '3' => "0011",
                    '4' => "0100",
                    '5' => "0101",
                    '6' => "0110",
                    '7' => "0111",
                    '8' => "1000",
                    '9' => "1001",
                    'A' => "1010",
                    'B' => "1011",
                    'C' => "1100",
                    'D' => "1101",
                    'E' => "1110",
                    'F' => "1111",
                    _ => "",
                }
                .chars()
            })
            .collect()
    }

    fn total_version(&self, bits: &str) -> (i64, usize) {
        debug!("bits {}", bits);
        let mut consumed = 0;
        let mut total_version = i64::from_str_radix(&bits[consumed..consumed + 3], 2).unwrap();
        consumed += 3;
        let packet_type = i32::from_str_radix(&bits[consumed..consumed + 3], 2).unwrap();
        consumed += 3;
        let value = match packet_type {
            4 => {
                // Literal value
                let mut value = "".to_string();
                loop {
                    let flag = i32::from_str_radix(&bits[consumed..consumed + 1], 2).unwrap();
                    consumed += 1;
                    value.push_str(&bits[consumed..consumed + 4]);
                    consumed += 4;
                    if flag == 0 {
                        break;
                    }
                    debug!("value: {}", value);
                }
                let value = i64::from_str_radix(&value, 2).unwrap();
                debug!("literal value: {}", value);
                value
            }
            _ => {
                // Operator
                let mut values = Vec::new();
                let length_type_id = i32::from_str_radix(&bits[consumed..consumed + 1], 2).unwrap();
                consumed += 1;
                match length_type_id {
                    0 => {
                        // 15 bit length
                        let mut length_subpackets =
                            usize::from_str_radix(&bits[consumed..consumed + 15], 2).unwrap();
                        consumed += 15;
                        while length_subpackets > 0 {
                            debug!("pre {}", &bits[consumed..]);
                            let (sub_value, used) = self.total_version(&bits[consumed..]);
                            values.push(sub_value);
                            consumed += used;
                            debug!("post {}", &bits[consumed..]);
                            length_subpackets -= used;
                        }
                        debug!("done {}", &bits[consumed..]);
                    }
                    1 => {
                        // 11 bit num_subpackets
                        let mut num_subpackets =
                            i32::from_str_radix(&bits[consumed..consumed + 11], 2).unwrap();
                        consumed += 11;
                        while num_subpackets > 0 {
                            debug!("num_subpackets {}", num_subpackets);
                            debug!("pre {}", &bits[consumed..]);
                            let (sub_value, used) = self.total_version(&bits[consumed..]);
                            values.push(sub_value);
                            consumed += used;
                            debug!("post {}", &bits[consumed..]);
                            num_subpackets -= 1;
                        }
                        debug!("done {}", &bits[consumed..]);
                    }
                    _ => unreachable!(),
                };
                match packet_type {
                    0 => values.into_iter().sum(),
                    1 => values.into_iter().product(),
                    2 => values.into_iter().min().unwrap(),
                    3 => values.into_iter().max().unwrap(),
                    5 => {
                        if values.get(0).unwrap() > values.get(1).unwrap() {
                            1
                        } else {
                            0
                        }
                    }
                    6 => {
                        if values.get(0).unwrap() < values.get(1).unwrap() {
                            1
                        } else {
                            0
                        }
                    }
                    7 => {
                        if values.get(0).unwrap() == values.get(1).unwrap() {
                            1
                        } else {
                            0
                        }
                    }
                    _ => unreachable!(),
                }
            }
        };

        (value, consumed)
    }

    pub fn analyse(&mut self) {
        let bits = self.bitstream();
        debug!("bits: {}", bits);
        self.answer = self.total_version(&bits).0;
    }

    pub fn answer(&self) -> i64 {
        self.answer
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[ctor::ctor]
    fn init() {
        env_logger::init();
    }

    #[test]
    fn version_total_1() {
        let mut solution = Solution::new();
        solution.set_input("C200B40A82".to_string());
        solution.analyse();
        assert_eq!(solution.answer(), 3);
    }

    #[test]
    fn version_total_2() {
        let mut solution = Solution::new();
        solution.set_input("04005AC33890".to_string());
        solution.analyse();
        assert_eq!(solution.answer(), 54);
    }

    #[test]
    fn version_total_3() {
        let mut solution = Solution::new();
        solution.set_input("880086C3E88112".to_string());
        solution.analyse();
        assert_eq!(solution.answer(), 7);
    }

    #[test]
    fn version_total_4() {
        let mut solution = Solution::new();
        solution.set_input("CE00C43D881120".to_string());
        solution.analyse();
        assert_eq!(solution.answer(), 9);
    }

    #[test]
    fn version_total_5() {
        let mut solution = Solution::new();
        solution.set_input("D8005AC2A8F0".to_string());
        solution.analyse();
        assert_eq!(solution.answer(), 1);
    }

    #[test]
    fn version_total_6() {
        let mut solution = Solution::new();
        solution.set_input("F600BC2D8F".to_string());
        solution.analyse();
        assert_eq!(solution.answer(), 0);
    }

    #[test]
    fn version_total_7() {
        let mut solution = Solution::new();
        solution.set_input("9C005AC2F8F0".to_string());
        solution.analyse();
        assert_eq!(solution.answer(), 0);
    }

    #[test]
    fn version_total_8() {
        let mut solution = Solution::new();
        solution.set_input("9C0141080250320F1802104A08".to_string());
        solution.analyse();
        assert_eq!(solution.answer(), 1);
    }
}
