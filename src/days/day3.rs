use regex::Regex;


struct ElfComputer<'a> {
    pc: usize,
    mul_enabled_flag: bool,
    rom: &'a [Instruction],
    sum_reg: i32,
}

impl<'a> ElfComputer<'a> {
    pub fn new(rom: &'a [Instruction]) -> Self {
        Self {
            pc: 0,
            mul_enabled_flag: true,
            rom,
            sum_reg: 0,
        }
    }

    pub fn run(&mut self) {
        while self.pc < self.rom.len() {
            let instr = &self.rom[self.pc];
            self.execute(instr);
            self.pc += 1;
        }
    }

    pub fn execute(&mut self, instr: &Instruction) {
        match instr {
            Instruction::Mul(lhs, rhs) => {
                if self.mul_enabled_flag {
                    self.sum_reg += lhs * rhs;
                }
            },
            Instruction::Do => self.mul_enabled_flag = true,
            Instruction::Dont => self.mul_enabled_flag = false,
        }
    }
}


#[derive(Debug, Clone, Copy)]
enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

impl Instruction {
    pub fn from_input(mut input: &str) -> Vec<Self> {
        let mut v = Vec::new();
        while let Some(i) = Self::parse_next(&mut input) {
            v.push(i);
        }
        v
    }

    pub fn parse_next(input: &mut &str) -> Option<Self> {
        while !input.is_empty() {
            if let Some(instr) = Self::match_do(input) {
                return Some(instr);
            }

            if let Some(instr) = Self::match_dont(input) {
                return Some(instr);
            }

            if let Some(instr) = Self::match_mul(input) {
                return Some(instr);
            }
            *input = &input[1..];
        }
        None
    }

    fn match_mul(input: &mut &str) -> Option<Self> {
        if !input.starts_with("mul(") {
            return None;
        }
        let mut chars = input.chars().skip(4); // skip past mul(

        // get idx count for first number
        let mut c = 0;
        for ch in chars.by_ref() {
            if ch == ',' {
                break;
            }
            else if !ch.is_numeric() {
                return None; // some other char was hit
            }

            c += 1;
        }

        // get idx count for second number
        let mut c2 = 0;
        for ch in chars {
            if ch == ')' {
                break;
            }
            else if !ch.is_numeric() {
                return None; // some other char was hit
            }
            c2 += 1;
        }

        let lhs = input[4..4+c].parse::<i32>().unwrap();
        let rhs = input[4+c+1..4+c+1+c2].parse::<i32>().unwrap();

        *input = input.split_at(4 + c+1+c2+1).1;

        let instr = Self::Mul(lhs, rhs);
        Some(instr)
    }

    fn match_do(input: &mut &str) -> Option<Self> {
        if !input.starts_with("do()") {
            return None;
        }
        *input = &input[4..];
        Some(Self::Do)
    }

    fn match_dont(input: &mut &str) -> Option<Self> {
        if !input.starts_with("don't()") {
            return None;
        }
        *input = &input[6..];
        Some(Self::Dont)
    }
}


pub(crate) struct Day3Executor<'a> {
    input: &'a str,
}

impl<'a> Day3Executor<'a> {
    const LINE_SEP: &'static str = "   ";

    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn exec_a(&mut self) -> String {
        let reg = Regex::new(r#"mul\((\d+),(\d+)\)"#).unwrap();
        reg.captures_iter(self.input)
        .map(|cap| cap.extract())
        .map(|(_, [lhs, rhs])| {
            let lhs = lhs.parse::<i32>().unwrap();
            let rhs = rhs.parse::<i32>().unwrap();
            lhs * rhs
        })
        .sum::<i32>()
        .to_string()
    }

    pub fn exec_b(&mut self) -> String {
        let instr = Instruction::from_input(self.input);
        let mut computer = ElfComputer::new(&instr);
        computer.run();
        computer.sum_reg.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::days::{Day, DayNumber};

    use super::*;

    #[test]
    fn test_example_a() {
        let mut day = Day::new(DayNumber::Day3);
        day.read_test_input();

        let timer = std::time::Instant::now();
        let res = day.exec_a();
        let t = timer.elapsed();
        println!("Result: {:?} ({:?})", res, t);
    }


    #[test]
    fn test_example_b() {
        let mut day = Day::new(DayNumber::Day3);
        day.read_test_input();

        let timer = std::time::Instant::now();
        let res = day.exec_b();
        let t = timer.elapsed();
        println!("Result: {:?} ({:?})", res, t);
    }
    
    #[test]
    fn test_parse_instr() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let instr = Instruction::from_input(input);
        println!("instr: {0:?}", instr);
    }

    #[test]
    fn test_parse_mul() {
        let input = "mul(123, 456)";
        let instr = Instruction::from_input(input);
        println!("instr: {0:?}", instr);
    }
}