use rayon::iter::{ParallelBridge, ParallelIterator};

pub(crate) struct Day2Executor<'a> {
    input: &'a str,
}

#[derive(Debug)]
struct Report {
    num: Vec<i32>,
}

impl Report {
    pub fn from_line(line: &str) -> Self {
        let num = line.split(Day2Executor::LINE_SEP)
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
        Self { num }
    }

    fn is_slice_safe(slice: &[i32]) -> bool {
        let inc = slice[1] > slice[0];

        for w in slice.windows(2) {
            let prev = w[0];
            let x = w[1];
            let order_wrong = match inc {
                true => prev > x,
                false => prev < x,
            };

            if order_wrong {
                return false;
            }

            let dif = (prev-x).abs();
            let diff_too_large = !(1..=3).contains(&dif);

            if diff_too_large {
                return false;
            }
        }

        true
    }

    pub fn is_safe(&self) -> bool {
        Self::is_slice_safe(&self.num)
    }

    /// Check safety with 1 level removable
    pub fn is_safe_b(&self) -> bool {
        // fuck ass bruteforcing
        (0..self.num.len())
        .any(|i| {
            let (l, r) = self.num.split_at(i);
            let conc = [l, &r[1..]].concat();
            // println!("{0:?}", conc);
            Self::is_slice_safe(&conc)
        })
    }
}


impl<'a> Day2Executor<'a> {
    const LINE_SEP: &'static str = " ";

    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn exec_a(&mut self) -> String {
        self.input.lines()
        .map(Report::from_line)
        .filter(|r| r.is_safe())
        .count()
        .to_string()
    }

    pub fn exec_b(&mut self) -> String {
        self.input.lines()
        .map(Report::from_line)
        .filter(|r| r.is_safe_b())
        .count()
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_line() {
        let line = "14 12 9 6 4 3 1";
        let report = Report::from_line(line).is_safe();
        println!("report: {0:?}", report);
        // assert!(!report);
    }

    #[test]
    fn test_removable() {
        let line = "13 12 10 8 6 4";
        let report = Report::from_line(line).is_safe_b();
        println!("report: {0:?}", report);

        let line = "11 12 10 8 6 5";
        let report = Report::from_line(line).is_safe_b();
        println!("report: {0:?}", report);
    }
}