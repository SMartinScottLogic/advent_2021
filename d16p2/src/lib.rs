use log::debug;
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

#[derive(Debug)]
enum PacketInner {
    None,
    Literal(i64),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Min(Vec<Packet>),
    Max(Vec<Packet>),
    GreaterThan(Vec<Packet>),
    LessThan(Vec<Packet>),
    Equals(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: i64,
    packet_inner: PacketInner,
}

impl FromStr for Packet {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bits = Self::hex_to_bin(s);
        Ok(Self::from_bin_str(&bits).0)
    }
}

impl Packet {
    fn empty() -> Packet {
        Packet {
            version: 0,
            packet_inner: PacketInner::None,
        }
    }

    fn calculate(&self) -> i64 {
        use PacketInner::*;
        match &self.packet_inner {
            Literal(value) => value.to_owned(),
            Sum(sub_packets) => sub_packets.iter().map(|packet| packet.calculate()).sum(),
            Product(sub_packets) => sub_packets
                .iter()
                .map(|packet| packet.calculate())
                .product(),
            Min(sub_packets) => sub_packets
                .iter()
                .map(|packet| packet.calculate())
                .min()
                .unwrap(),
            Max(sub_packets) => sub_packets
                .iter()
                .map(|packet| packet.calculate())
                .max()
                .unwrap(),
            GreaterThan(sub_packets) => {
                if sub_packets.get(0).unwrap().calculate() > sub_packets.get(1).unwrap().calculate()
                {
                    1
                } else {
                    0
                }
            }
            LessThan(sub_packets) => {
                if sub_packets.get(0).unwrap().calculate() < sub_packets.get(1).unwrap().calculate()
                {
                    1
                } else {
                    0
                }
            }
            Equals(sub_packets) => {
                if sub_packets.get(0).unwrap().calculate()
                    == sub_packets.get(1).unwrap().calculate()
                {
                    1
                } else {
                    0
                }
            }
            None => 0,
        }
    }

    fn hex_to_bin(input: &str) -> String {
        input
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
                    _ => unreachable!(),
                }
                .chars()
            })
            .collect()
    }

    fn child_packets(bits: &str) -> (Vec<Packet>, usize) {
        let mut values = Vec::new();
        let length_type_id = i32::from_str_radix(&bits[0..1], 2).unwrap();
        let mut consumed = 1;
        match length_type_id {
            0 => {
                // 15 bit length
                let mut length_subpackets =
                    usize::from_str_radix(&bits[consumed..consumed + 15], 2).unwrap();
                consumed += 15;
                while length_subpackets > 0 {
                    debug!("pre {}", &bits[consumed..]);
                    let (sub_value, used) = Self::from_bin_str(&bits[consumed..]);
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
                    let (sub_value, used) = Self::from_bin_str(&bits[consumed..]);
                    values.push(sub_value);
                    consumed += used;
                    debug!("post {}", &bits[consumed..]);
                    num_subpackets -= 1;
                }
                debug!("done {}", &bits[consumed..]);
            }
            _ => unreachable!(),
        };
        (values, consumed)
    }

    fn from_bin_str(bits: &str) -> (Packet, usize) {
        use PacketInner::*;
        let mut consumed = 0;
        let version = i64::from_str_radix(&bits[consumed..consumed + 3], 2).unwrap();
        consumed += 3;
        let packet_type = i32::from_str_radix(&bits[consumed..consumed + 3], 2).unwrap();
        consumed += 3;
        debug!("packet_type: {}", packet_type);
        let packet = match packet_type {
            0 => {
                let (child_packets, child_packets_consumed) =
                    Self::child_packets(&bits[consumed..]);
                consumed += child_packets_consumed;
                Packet {
                    version,
                    packet_inner: Sum(child_packets),
                }
            }
            1 => {
                let (child_packets, child_packets_consumed) =
                    Self::child_packets(&bits[consumed..]);
                consumed += child_packets_consumed;
                Packet {
                    version,
                    packet_inner: Product(child_packets),
                }
            }
            2 => {
                let (child_packets, child_packets_consumed) =
                    Self::child_packets(&bits[consumed..]);
                consumed += child_packets_consumed;
                Packet {
                    version,
                    packet_inner: Min(child_packets),
                }
            }
            3 => {
                let (child_packets, child_packets_consumed) =
                    Self::child_packets(&bits[consumed..]);
                consumed += child_packets_consumed;
                Packet {
                    version,
                    packet_inner: Max(child_packets),
                }
            }
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
                Packet {
                    version,
                    packet_inner: PacketInner::Literal(value),
                }
            }
            5 => {
                let (child_packets, child_packets_consumed) =
                    Self::child_packets(&bits[consumed..]);
                consumed += child_packets_consumed;
                Packet {
                    version,
                    packet_inner: GreaterThan(child_packets),
                }
            }
            6 => {
                let (child_packets, child_packets_consumed) =
                    Self::child_packets(&bits[consumed..]);
                consumed += child_packets_consumed;
                Packet {
                    version,
                    packet_inner: LessThan(child_packets),
                }
            }
            7 => {
                let (child_packets, child_packets_consumed) =
                    Self::child_packets(&bits[consumed..]);
                consumed += child_packets_consumed;
                Packet {
                    version,
                    packet_inner: Equals(child_packets),
                }
            }
            _ => unreachable!(),
        };
        (packet, consumed)
    }
}

#[derive(Debug)]
pub struct Solution {
    packet: Packet,
    answer: i64,
}

impl Solution {
    fn new() -> Self {
        Self {
            packet: Packet::empty(),
            answer: 0,
        }
    }

    fn set_input(&mut self, input: String) {
        self.packet = Packet::from_str(&input).unwrap();
        debug!("{:?}", self);
    }

    pub fn analyse(&mut self) {
        self.answer = self.packet.calculate();
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
