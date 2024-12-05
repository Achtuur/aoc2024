use std::collections::{hash_map::Entry, HashMap};

pub(crate) struct Day1Executor<'a> {
    input: &'a str,
}

impl<'a> Day1Executor<'a> {
    const LINE_SEP: &'static str = "   ";

    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn exec_a(&mut self) -> String {
        let (mut list_left, mut list_right) = self.input
        .lines()
        .map(|x| {
            let mut iter = x.split(Self::LINE_SEP);
            let left = iter.next().unwrap().parse::<i32>().unwrap();
            let right = iter.next().unwrap().parse::<i32>().unwrap();
            (left, right)
        })
        .collect::<(Vec<i32>, Vec<i32>)>();
        list_left.sort();
        list_right.sort();

        let mut res = 0;
        for (l, r) in list_left.iter().zip(list_right.iter()) {
            res += (l - r).abs();
        }
        res.to_string()
    }

    pub fn exec_b(&self) -> String {
        let (lhs_numbers, rhs_occurences) = self.input
        .lines()
        .fold((Vec::new(), HashMap::new()), |(mut lhs, mut rhs_map), line| {
            let mut split = line.split(Self::LINE_SEP);
            let left = split.next().unwrap().parse::<i32>().unwrap();
            let right = split.next().unwrap().parse::<i32>().unwrap();

            lhs.push(left);
            match rhs_map.entry(right) {
                Entry::Occupied(mut entry) => *entry.get_mut() += 1,
                Entry::Vacant(entry) => { entry.insert(1); },
            }
            (lhs, rhs_map)
        });

        lhs_numbers.into_iter().fold(0, |sum, x| {
            sum + x * rhs_occurences.get(&x).unwrap_or(&0)
        }).to_string()
    }
}
