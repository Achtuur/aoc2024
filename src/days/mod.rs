use std::path::{Path, PathBuf};

mod day1;
mod day2;
mod day3;
mod day4;

use day1::Day1Executor;
use day2::Day2Executor;
use day3::Day3Executor;
use day4::Day4Executor;

#[allow(unused)]
pub enum DayNumber {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
}

pub struct Day {
    number: DayNumber,
    input: String,
}

impl Day {
    pub fn new(number: DayNumber) -> Self {
        Self {
            number,
            input: String::new(),
        }
    }

    #[cfg(test)]
    pub fn read_test_input(&mut self) {
        let path = self.example_input_path();
        self.read_input_from_file(path);
    }

    pub fn read_input(&mut self) {
        let path = self.input_path();
        self.read_input_from_file(path);
    }

    fn read_input_from_file(&mut self, path: impl AsRef<Path>) {
        let input = std::fs::read_to_string(path).unwrap();
        self.input = input;
    }

    pub fn exec_a(&mut self) -> String {
        match self.number {
            DayNumber::Day1 => Day1Executor::new(&self.input).exec_a(),
            DayNumber::Day2 => Day2Executor::new(&self.input).exec_a(),
            DayNumber::Day3 => Day3Executor::new(&self.input).exec_a(),
            DayNumber::Day4 => Day4Executor::new(&self.input).exec_a(),
            DayNumber::Day5 => todo!(),
            DayNumber::Day6 => todo!(),
            DayNumber::Day7 => todo!(),
            DayNumber::Day8 => todo!(),
            DayNumber::Day9 => todo!(),
            DayNumber::Day10 => todo!(),
            DayNumber::Day11 => todo!(),
            DayNumber::Day12 => todo!(),
            DayNumber::Day13 => todo!(),
            DayNumber::Day14 => todo!(),
        }
    }

    pub fn exec_b(&mut self) -> String {
        match self.number {
            DayNumber::Day1 => Day1Executor::new(&self.input).exec_b(),
            DayNumber::Day2 => Day2Executor::new(&self.input).exec_b(),
            DayNumber::Day3 => Day3Executor::new(&self.input).exec_b(),
            DayNumber::Day4 => Day4Executor::new(&self.input).exec_b(),
            DayNumber::Day5 => todo!(),
            DayNumber::Day6 => todo!(),
            DayNumber::Day7 => todo!(),
            DayNumber::Day8 => todo!(),
            DayNumber::Day9 => todo!(),
            DayNumber::Day10 => todo!(),
            DayNumber::Day11 => todo!(),
            DayNumber::Day12 => todo!(),
            DayNumber::Day13 => todo!(),
            DayNumber::Day14 => todo!(),
        }
    }

    fn input_path(&self) -> PathBuf {
        PathBuf::from(format!("inputs/{}/input.txt", self.to_string()))
    }

    fn example_input_path(&self) -> PathBuf {
        PathBuf::from(format!("inputs/{}/example.txt", self.to_string()))
    }

    fn to_string(&self) -> &'static str {
        match self.number {
            DayNumber::Day1 => "day1",
            DayNumber::Day2 => "day2",
            DayNumber::Day3 => "day3",
            DayNumber::Day4 => "day4",
            DayNumber::Day5 => "day5",
            DayNumber::Day6 => "day6",
            DayNumber::Day7 => "day7",
            DayNumber::Day8 => "day8",
            DayNumber::Day9 => "day9",
            DayNumber::Day10 => "day10",
            DayNumber::Day11 => "day11",
            DayNumber::Day12 => "day12",
            DayNumber::Day13 => "day13",
            DayNumber::Day14 => "day14",
        }
    }
}