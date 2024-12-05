
struct Grid {
    width: usize,
    data: Vec<char>,
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let mut width = 0;
        let grid = input.lines().flat_map(|line| {
            width = line.len();
            line.chars()
        }).collect::<Vec<char>>();

        Self {
            width,
            data: grid
        }
    }

    pub fn i_to_xy(&self, i: usize) -> (usize, usize) {
        (i % self.width, i / self.width)
    }

    pub fn get(&self, x: usize, y: usize) -> char {
        self.data[y * self.width + x]
    }

    pub fn get_i(&self, i: usize) -> char {
        self.data[i]
    }

    pub fn search_str(&self, x: usize, y: usize, pat: &str) -> usize {
        
    }
}

pub(crate) struct Day4Executor<'a> {
    input: &'a str,
}

impl<'a> Day4Executor<'a> {
    const LINE_SEP: &'static str = "   ";

    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn exec_a(&mut self) -> String {
        let grid = Grid::new(self.input);

        String::new()
    }

    pub fn exec_b(&mut self) -> String {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::days::{Day, DayNumber};

    use super::*;

    #[test]
    fn test_example_a() {
        let mut day = Day::new(DayNumber::Day4);
        day.read_test_input();

        let timer = std::time::Instant::now();
        let res = day.exec_a();
        let t = timer.elapsed();
        println!("Result: {:?} ({:?})", res, t);
    }


    #[test]
    fn test_example_b() {
        let mut day = Day::new(DayNumber::Day4);
        day.read_test_input();

        let timer = std::time::Instant::now();
        let res = day.exec_b();
        let t = timer.elapsed();
        println!("Result: {:?} ({:?})", res, t);
    }
}