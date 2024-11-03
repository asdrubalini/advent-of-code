use itertools::Itertools;

use crate::aoc::Solution;

pub struct TwentyThree;

#[derive(Debug, Clone, Copy)]
pub enum Register {
    A,
    B,
}

impl From<&str> for Register {
    fn from(value: &str) -> Self {
        match value {
            "a" => Register::A,
            "b" => Register::B,
            _ => panic!("oh no"),
        }
    }
}

type Offset = isize;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(Offset),
    Jie(Register, Offset),
    Jio(Register, Offset),
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        let mut pieces = line.split_ascii_whitespace();

        let instruction = pieces.next().unwrap();
        let data = pieces.join(" ");

        fn parse_comma_separated(s: &str) -> (Register, Offset) {
            let (a, b) = s.split(", ").collect_tuple().unwrap();
            (Register::from(a), b.parse().unwrap())
        }

        match instruction {
            "hlf" => Instruction::Hlf(Register::from(data.as_str())),
            "tpl" => Instruction::Tpl(Register::from(data.as_str())),
            "inc" => Instruction::Inc(Register::from(data.as_str())),
            "jmp" => Instruction::Jmp(data.parse().unwrap()),
            "jie" => Instruction::Jie(
                parse_comma_separated(&data).0,
                parse_comma_separated(&data).1,
            ),
            "jio" => Instruction::Jio(
                parse_comma_separated(&data).0,
                parse_comma_separated(&data).1,
            ),
            _ => panic!("Unknown instruction"),
        }
    }
}

pub struct CPU {
    a: u32,
    b: u32,
    instructions: Vec<Instruction>,
    pc: usize,
}

impl CPU {
    fn new(instructions: &[Instruction], a: u32, b: u32) -> Self {
        CPU {
            a,
            b,
            instructions: instructions.to_owned(),
            pc: 0,
        }
    }

    fn jump(&mut self, offset: Offset) {
        self.pc = (self.pc as isize).checked_add(offset).unwrap() as usize;
    }

    fn step(&mut self) -> Result<(), ()> {
        let inst = *self.instructions.get(self.pc).ok_or(())?;

        match inst {
            Instruction::Hlf(register) => match register {
                Register::A => self.a /= 2,
                Register::B => self.b /= 2,
            },
            Instruction::Tpl(register) => match register {
                Register::A => self.a *= 3,
                Register::B => self.b *= 3,
            },
            Instruction::Inc(register) => match register {
                Register::A => self.a += 1,
                Register::B => self.b += 1,
            },
            Instruction::Jmp(offset) => self.jump(offset),
            Instruction::Jie(register, offset) => {
                // jump if even

                let val = match register {
                    Register::A => self.a,
                    Register::B => self.b,
                };

                if val % 2 == 0 {
                    self.jump(offset)
                } else {
                    self.pc += 1;
                }
            }
            Instruction::Jio(register, offset) => {
                // jump if one

                let val = match register {
                    Register::A => self.a,
                    Register::B => self.b,
                };

                if val == 1 {
                    self.jump(offset)
                } else {
                    self.pc += 1;
                }
            }
        };

        match inst {
            Instruction::Hlf(_) | Instruction::Tpl(_) | Instruction::Inc(_) => self.pc += 1,
            _ => (),
        };

        println!(
            "{:?}, pc = {}, a = {}, b = {}",
            inst, self.pc, self.a, self.b
        );

        Ok(())
    }
}

impl Solution for TwentyThree {
    type Output = u32;
    type Parsed = Vec<Instruction>;

    fn input() -> &'static str {
        include_str!("../inputs/23.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        input.lines().map(Instruction::from).collect()
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        let mut cpu = CPU::new(&parsed, 0, 0);

        while let Ok(_) = cpu.step() {}

        cpu.b
    }

    fn solve_second(parsed: &Self::Parsed) -> Self::Output {
        let mut cpu = CPU::new(&parsed, 1, 0);

        while let Ok(_) = cpu.step() {}

        cpu.b
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (255, 334)
    }
}
