use anyhow::{Context, Result};
use log::{debug, trace};
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::ops::Add;
use std::str::FromStr;

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename).with_context(|| format!("Failed to read from {}", filename))?;

    let reader = BufReader::new(file);

    let mut solution = Solution::new();
    for line in reader.lines() {
        solution = solution + SnailfishNumber::from_str(&line?)?;
    }
    Ok(solution)
}

#[derive(Debug)]
pub struct Solution {
    data: Option<SnailfishNumber>,
    answer: i64,
}

impl Solution {
    fn new() -> Self {
        Self {
            data: None,
            answer: 0,
        }
    }

    pub fn analyse(&mut self) {
        debug!("sum: {}", self.data.as_ref().unwrap().to_string());
        self.answer = self.data.as_ref().unwrap().magnitude();
    }

    pub fn answer(&self) -> Result<i64> {
        Ok(self.answer)
    }
}

impl Add<SnailfishNumber> for Solution {
    type Output = Self;

    fn add(self, other: SnailfishNumber) -> Self {
        let o = format!(
            "{:?} + {:?}",
            self.data
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_else(|| "[]".to_string()),
            other.to_string()
        );
        let sum = match self.data {
            Some(data) => SnailfishNumber::Pair(Box::new((data, other))),
            None => other,
        };
        debug!("{}", sum.to_string());
        let sum = SnailfishNumber::reduce(sum);
        debug!("{} = {:?}", o, sum.to_string());
        Self {
            data: Some(sum),
            answer: 0,
        }
    }
}

#[derive(Debug, PartialEq)]
enum SnailfishNumber {
    Number(usize),
    Pair(Box<(SnailfishNumber, SnailfishNumber)>),
}

impl SnailfishNumber {
    fn magnitude(&self) -> i64 {
        match self {
            Self::Number(v) => *v as i64,
            Self::Pair(p) => {
                let lhs = &p.0;
                let rhs = &p.1;
                let lhs = lhs.magnitude();
                let rhs = rhs.magnitude();
                lhs * 3 + rhs * 2
            }
        }
    }
}

impl SnailfishNumber {
    fn explode_worker(
        value: SnailfishNumber,
        depth: usize,
        increment_left: usize,
        increment_right: usize,
        immutable: bool,
    ) -> (SnailfishNumber, usize, usize, bool) {
        trace!(
            "Initial: ({} {} {} {}) {}",
            value.to_string(),
            increment_left,
            increment_right,
            immutable,
            depth
        );
        let value = match value {
            // Simple numbers handled at parent
            Self::Number(_) => unreachable!(),
            // Should explode
            Self::Pair(p) if depth == 4 && !immutable => match *p {
                (SnailfishNumber::Number(lhs), SnailfishNumber::Number(rhs)) => {
                    (SnailfishNumber::Number(0), lhs, rhs, true)
                }
                _ => unreachable!(),
            },
            Self::Pair(p) => {
                let lhs = p.0;
                let rhs = p.1;
                trace!(
                    "lhs {} {} {} {}",
                    lhs.to_string(),
                    increment_left,
                    increment_right,
                    immutable
                );
                let (lhs, pass_left, lpr, lc) = match lhs {
                    Self::Number(v) => (Self::Number(v + increment_left), 0, 0, immutable),
                    Self::Pair(_) => {
                        let (lhs, pl, pr, changed) =
                            Self::explode_worker(lhs, depth + 1, increment_left, 0, immutable);
                        (lhs, pl, pr, changed)
                    }
                };
                trace!("lhs1 {} {} {} {}", lhs.to_string(), pass_left, lpr, lc);

                let (rhs, rpl, pass_right, rc) = match rhs {
                    Self::Number(v) => (Self::Number(v + lpr + increment_right), 0, 0, lc),
                    Self::Pair(_) => Self::explode_worker(rhs, depth + 1, lpr, increment_right, lc),
                };
                trace!("rhs {} {} {} {}", rhs.to_string(), rpl, pass_right, rc);
                let (lhs, pass_left, pass_right, changed) = if rc != lc {
                    trace!("scatter into lhs");
                    match lhs {
                        Self::Number(v) if rpl != 0 => {
                            (Self::Number(v + rpl), pass_left, pass_right, true)
                        }
                        Self::Pair(_) if rpl != 0 => {
                            let (lhs, _, _, _) = Self::explode_worker(lhs, depth + 1, 0, rpl, true);
                            (lhs, pass_left, pass_right, true)
                        }
                        v => (v, pass_left, pass_right, true),
                    }
                } else {
                    (lhs, pass_left, pass_right, lc)
                };
                trace!(
                    "lhs2 {} {} {} {} {}",
                    lhs.to_string(),
                    rhs.to_string(),
                    pass_left,
                    pass_right,
                    changed
                );

                (
                    Self::Pair(Box::new((lhs, rhs))),
                    pass_left,
                    pass_right,
                    changed,
                )
            }
        };
        trace!(
            "Final: ({} {} {} {}) {}",
            value.0.to_string(),
            value.1,
            value.2,
            value.3,
            depth
        );
        value
    }

    fn explode(value: SnailfishNumber) -> (SnailfishNumber, bool) {
        let (value, _, _, changed) = Self::explode_worker(value, 0, 0, 0, false);
        (value, changed)
    }

    fn split_worker(value: SnailfishNumber, immutable: bool) -> (SnailfishNumber, bool) {
        match value {
            Self::Number(v) if v >= 10 && !immutable => {
                let lv = v / 2;
                let rv = v - lv;
                (
                    SnailfishNumber::Pair(Box::new((
                        SnailfishNumber::Number(lv),
                        SnailfishNumber::Number(rv),
                    ))),
                    true,
                )
            }
            Self::Pair(p) if !immutable => {
                let lhs = p.0;
                let rhs = p.1;
                let (lhs, changed) = Self::split_worker(lhs, immutable);
                let (rhs, changed) = if !changed {
                    Self::split_worker(rhs, changed)
                } else {
                    (rhs, changed)
                };
                (Self::Pair(Box::new((lhs, rhs))), changed)
            }
            _ => (value, immutable),
        }
    }

    fn split(value: SnailfishNumber) -> (SnailfishNumber, bool) {
        Self::split_worker(value, false)
    }

    fn reduce(mut value: Self) -> Self {
        loop {
            let (new_value, changed) = Self::explode(value);
            if changed {
                debug!("explode -> {}", new_value.to_string());
                value = new_value;
                continue;
            }
            let (new_value, changed) = Self::split(new_value);
            if changed {
                debug!("split -> {}", new_value.to_string());
                value = new_value;
                continue;
            }
            value = new_value;
            break;
        }
        value
    }
}

impl SnailfishNumber {
    fn split_str(value: &str) -> (&str, &str) {
        let start = 1;
        let end = value.len() - 1;
        let mut depth = 0;
        let mut mid = 0;
        for (pos, c) in value.chars().enumerate() {
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                ',' if depth == 1 => {
                    mid = pos;
                    break;
                }
                _ => {}
            }
        }
        let left = &value[start..=mid - 1];
        let right = &value[mid + 1..end];
        (left, right)
    }
}

impl ToString for SnailfishNumber {
    fn to_string(&self) -> String {
        match self {
            Self::Number(n) => n.to_string(),
            Self::Pair(p) => {
                let left = &p.0;
                let right = &p.1;
                format!("[{},{}]", left.to_string(), right.to_string())
            }
        }
    }
}

impl FromStr for SnailfishNumber {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let re = Regex::new(r"^[0-9]+$").unwrap();
        if re.is_match(s) {
            Ok(Self::Number(s.parse().unwrap()))
        } else {
            let (left, right) = Self::split_str(s);
            let left = Self::from_str(left).unwrap();
            let right = Self::from_str(right).unwrap();
            Ok(Self::Pair(Box::new((left, right))))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::str::FromStr;

    #[ctor::ctor]
    fn init() {
        env_logger::init();
    }

    fn test_explode_worker(
        input: &str,
        depth: usize,
        expected: &str,
        pass_left: usize,
        pass_right: usize,
        change: bool,
    ) {
        let input = SnailfishNumber::from_str(input).unwrap();
        let result = SnailfishNumber::explode_worker(input, depth, 0, 0, false);
        assert_eq!(
            result,
            (
                SnailfishNumber::from_str(expected).unwrap(),
                pass_left,
                pass_right,
                change
            )
        );
    }

    #[test]
    fn explode_worker() {
        test_explode_worker("[2,3]", 4, "0", 2, 3, true);
        test_explode_worker("[[2,3],[3,4]]", 3, "[0,[6,4]]", 2, 0, true);
        test_explode_worker("[2,[3,4]]", 3, "[5,0]", 0, 4, true);
        test_explode_worker(
            r"[[[[4,0],[5,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
            0,
            r"[[[[4,0],[5,4]],[[0,[7,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
            0,
            0,
            true,
        );
    }

    fn test_explode(src: &str, expected: &str) {
        let src = SnailfishNumber::from_str(src).unwrap();
        let expected = SnailfishNumber::from_str(expected).unwrap();
        assert_eq!(SnailfishNumber::explode(src).0, expected);
    }

    fn test_split_str(src: &str, expected: (&str, &str)) {
        let actual = SnailfishNumber::split_str(src);
        assert_eq!(actual, expected);
    }
    #[test]
    fn split_str() {
        test_split_str("[1,2]", ("1", "2"));
        test_split_str("[1,[2,3]]", ("1", "[2,3]"));
        test_split_str("[[1,2],3]", ("[1,2]", "3"));
        test_split_str(r"[7,[6,[5,[4,[3,2]]]]]", ("7", "[6,[5,[4,[3,2]]]]"));
        test_split_str(
            r"[[2,[3,[4,5]]],[[[0,1],2],3]]",
            (r"[2,[3,[4,5]]]", r"[[[0,1],2],3]"),
        );
    }

    #[test]
    fn explode() {
        test_explode(r"[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
        test_explode(r"[7,[6,[5,[4,[3,2]]]]]", r"[7,[6,[5,[7,0]]]]");
        test_explode(
            r"[[2,[3,[4,[3,2]]]],[[[0,1],2],3]]",
            r"[[2,[3,[7,0]]],[[[2,1],2],3]]",
        );
        test_explode("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
        test_explode("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
        test_explode("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
        test_explode(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        );
        test_explode(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        );
    }

    fn test_addition(lhs: &str, rhs: &str, expected: &str) {
        let mut solution = Solution::new();
        solution = solution + SnailfishNumber::from_str(lhs).unwrap();
        solution = solution + SnailfishNumber::from_str(rhs).unwrap();
        assert_eq!(
            solution.data.unwrap(),
            SnailfishNumber::from_str(expected).unwrap()
        );
    }

    fn test_split(src: &str, expected: &str) {
        let src = SnailfishNumber::from_str(src).unwrap();
        let expected = SnailfishNumber::from_str(expected).unwrap();
        assert_eq!(SnailfishNumber::split(src).0, expected);
    }

    #[test]
    fn split() {
        test_split(r"[1,2]", r"[1,2]");
        test_split(r"[15,5]", r"[[7,8],5]");
        test_split(r"[15,15]", r"[[7,8],15]");
    }

    #[test]
    fn addition() {
        test_addition(
            r"[[[[4,3],4],4],[7,[[8,4],9]]]",
            r"[1,1]",
            r"[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        );
        test_list_addition(
            vec![r"[1,1]", r"[2,2]", r"[3,3]", r"[4,4]"],
            r"[[[[1,1],[2,2]],[3,3]],[4,4]]",
        );
        test_list_addition(
            vec![r"[1,1]", r"[2,2]", r"[3,3]", r"[4,4]", r"[5,5]"],
            r"[[[[3,0],[5,3]],[4,4]],[5,5]]",
        );
        test_list_addition(
            vec![r"[1,1]", r"[2,2]", r"[3,3]", r"[4,4]", r"[5,5]", r"[6,6]"],
            r"[[[[5,0],[7,4]],[5,5]],[6,6]]",
        );
    }

    fn test_magnitude(input: &str, expected: i64) {
        assert_eq!(
            SnailfishNumber::from_str(input).unwrap().magnitude(),
            expected
        );
    }

    #[test]
    fn magnitude() {
        test_magnitude("[[1,2],[[3,4],5]]", 143);
        test_magnitude("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384);
        test_magnitude("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445);
        test_magnitude("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791);
        test_magnitude("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137);
        test_magnitude(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            3488,
        );
    }

    #[test]
    fn list_magnitude() {
        test_list_magnitude(
            vec![
                r"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
                r"[[[5,[2,8]],4],[5,[[9,9],0]]]",
                r"[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
                r"[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
                r"[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
                r"[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
                r"[[[[5,4],[7,7]],8],[[8,3],8]]",
                r"[[9,3],[[9,9],[6,[4,9]]]]",
                r"[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
                r"[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
            ],
            r"[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]",
            4140,
        );

        test_list_addition(
            vec![
                r"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
                r"[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            ],
            r"[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
        );

        test_list_magnitude(
            vec![
                r"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
                r"[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
                r"[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
                r"[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
                r"[7,[5,[[3,8],[1,4]]]]",
                r"[[2,[2,2]],[8,[8,1]]]",
                r"[2,9]",
                r"[1,[[[9,3],9],[[9,0],[0,7]]]]",
                r"[[[5,[7,4]],7],1]",
                r"[[[[4,2],2],6],[8,7]]",
            ],
            r"[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            3488,
        );
    }

    fn test_list_addition(input: Vec<&str>, expected: &str) {
        let expected = SnailfishNumber::from_str(expected).unwrap();
        let solution = input.into_iter().fold(Solution::new(), |mut acc, v| {
            acc = acc + SnailfishNumber::from_str(v).unwrap();
            acc
        });
        assert_eq!(solution.data.unwrap(), expected);
    }

    fn test_list_magnitude(input: Vec<&str>, expected_sum: &str, expected: i64) {
        let expected_sum = SnailfishNumber::from_str(expected_sum).unwrap();
        let solution = input.into_iter().fold(Solution::new(), |mut acc, v| {
            acc = acc + SnailfishNumber::from_str(v).unwrap();
            acc
        });
        let result = solution.data.unwrap();
        assert_eq!(result, expected_sum);
        assert_eq!(result.magnitude(), expected);
    }
}
