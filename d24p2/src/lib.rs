use anyhow::{Context, Result};
use log::{debug, error};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

type Register = i64;

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename)?;

    let mut solution = Solution::new(true);
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let instruction = Instruction::from_str(&line?)?;
        solution.add_instruction(instruction);
    }

    Ok(solution)
}

#[derive(Debug)]
pub struct Solution {
    answer: Option<i64>,
    instructions: Vec<Instruction>,
    visited: HashMap<(Alu, usize), Option<i64>>,
    smallest: bool,
}

impl Solution {
    fn new(smallest: bool) -> Self {
        Self {
            answer: None,
            instructions: Vec::new(),
            visited: HashMap::new(),
            smallest,
        }
    }

    fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn analyse(&mut self) {
        self.answer = self.best(0, Alu::default()).map(|v| {
            v.to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse()
                .unwrap()
        });
    }

    pub fn answer(&self) -> Result<i64> {
        self.answer.context("No solution")
    }
}

impl Solution {
    fn best(&mut self, pc: usize, reg: Alu) -> Option<i64> {
        assert!(matches!(self.instructions[pc], Instruction::Inp(_)));

        if let Some(answer) = self.visited.get(&(reg, pc)) {
            return *answer;
        }

        let range = if self.smallest {
            [1, 2, 3, 4, 5, 6, 7, 8, 9]
        } else {
            [9, 8, 7, 6, 5, 4, 3, 2, 1]
        };
        'inputs: for input in range {
            let mut reg = reg;
            let mut pc = pc;
            if let Instruction::Inp(p) = self.instructions[pc] {
                match p {
                    Param::W => reg.w = input,
                    Param::X => reg.x = input,
                    Param::Y => reg.y = input,
                    Param::Z => reg.z = input,
                    _ => unreachable!(),
                }
            } else {
                panic!("Should be Inp @ {}", pc);
            }

            pc += 1;

            while let Some(inst) = self.instructions.get(pc) {
                if matches!(self.instructions[pc], Instruction::Inp(_)) {
                    if let Some(best) = self.best(pc, reg) {
                        self.visited.insert((reg, pc), Some(best * 10 + input));
                        return Some(best * 10 + input);
                    } else {
                        continue 'inputs;
                    }
                } else {
                    reg.apply_instruction(inst);
                    pc += 1;
                }
            }

            if reg.z == 0 {
                self.visited.insert((reg, pc), Some(input));
                return Some(input);
            }
        }

        self.visited.insert((reg, pc), None);
        None
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
struct Alu {
    w: Register,
    x: Register,
    y: Register,
    z: Register,
}

impl Alu {
    fn apply_instruction(&mut self, instruction: &Instruction) {
        use Instruction::*;
        debug!("apply {:?} to {:?}", instruction, self);
        match instruction {
            Add(p1, p2) => {
                let result = self.value(p1) + self.value(p2);
                self.set_value(p1, result);
            }
            Mul(p1, p2) => {
                let result = self.value(p1) * self.value(p2);
                self.set_value(p1, result);
            }
            Div(p1, p2) => {
                let result = self.value(p1) / self.value(p2);
                self.set_value(p1, result);
            }
            Mod(p1, p2) => {
                let result = self.value(p1) % self.value(p2);
                self.set_value(p1, result);
            }
            Eql(p1, p2) => {
                let result = if self.value(p1) == self.value(p2) {
                    1
                } else {
                    0
                };
                self.set_value(p1, result);
            }
            _ => {
                error!("unexpected instruction {:?}", instruction);
                unreachable!()
            }
        }
        debug!("  {:?}", self);
    }

    fn value(&self, param: &Param) -> Register {
        use Param::*;

        match param {
            Value(v) => *v,
            W => self.w,
            X => self.x,
            Y => self.y,
            Z => self.z,
        }
    }

    fn set_value(&mut self, param: &Param, result: Register) {
        use Param::*;

        match param {
            W => self.w = result,
            X => self.x = result,
            Y => self.y = result,
            Z => self.z = result,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Inp(Param),
    Add(Param, Param),
    Mul(Param, Param),
    Div(Param, Param),
    Mod(Param, Param),
    Eql(Param, Param),
}

#[derive(Debug, Clone, Copy)]
enum Param {
    Value(i64),
    W,
    X,
    Y,
    Z,
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Instruction::*;

        debug!("instruction from '{}'", s);

        let (op, params) =
            s.trim()
                .split(' ')
                .enumerate()
                .fold(("", Vec::new()), |mut acc, (idx, v)| {
                    match idx {
                        0 => acc.0 = v,
                        _ => acc.1.push(Param::from_str(v).unwrap()),
                    }
                    acc
                });
        let instruction = match op {
            "inp" => Inp(params[0]),
            "add" => Add(params[0], params[1]),
            "mul" => Mul(params[0], params[1]),
            "div" => Div(params[0], params[1]),
            "mod" => Mod(params[0], params[1]),
            "eql" => Eql(params[0], params[1]),
            _ => unreachable!(),
        };

        Ok(instruction)
    }
}

impl FromStr for Param {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Param::*;
        debug!("param from '{}'", s);

        let param = match s {
            "w" => W,
            "x" => X,
            "y" => Y,
            "z" => Z,
            _ => Value(s.parse().unwrap()),
        };
        Ok(param)
    }
}
