use log::debug;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
        match packet_type {
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
            }
            _ => {
                // Operator
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
                            let (sub_total_version, used) = self.total_version(&bits[consumed..]);
                            total_version += sub_total_version;
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
                            let (sub_total_version, used) = self.total_version(&bits[consumed..]);
                            debug!("post {}", &bits[consumed..]);
                            total_version += sub_total_version;
                            consumed += used;
                            num_subpackets -= 1;
                        }
                        debug!("done {}", &bits[consumed..]);
                    }
                    _ => unreachable!(),
                };
            }
        }

        (total_version, consumed)
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
        solution.set_input("D2FE28".to_string());
        solution.analyse();
        assert_eq!(solution.answer(), 6);
    }

    #[test]
    fn version_total_2() {
        let mut solution = Solution::new();
        solution.set_input("8A004A801A8002F478".to_string());
        solution.analyse();
        assert_eq!(solution.answer(), 16);
    }

    #[test]
    fn version_total_3() {
        let mut solution = Solution::new();
        solution.set_input("620080001611562C8802118E34".to_string());
        solution.analyse();
        assert_eq!(solution.answer(), 12);
    }

    #[test]
    fn version_total_4() {
        let mut solution = Solution::new();
        solution.set_input("C0015000016115A2E0802F182340".to_string());
        solution.analyse();
        assert_eq!(solution.answer(), 23);
    }

    #[test]
    fn version_total_5() {
        let mut solution = Solution::new();
        solution.set_input("A0016C880162017C3686B18A3D4780".to_string());
        solution.analyse();
        assert_eq!(solution.answer(), 31);
    }

    #[test]
    fn version_total_6() {
        let mut solution = Solution::new();
        solution.set_input("38006F45291200".to_string());
        solution.analyse();
        assert_eq!(solution.answer(), 9);
    }
}
