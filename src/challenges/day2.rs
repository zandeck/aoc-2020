use std::str::FromStr;

use crate::challenges::utils::read_file;
use regex;

#[derive(Debug)]
pub struct Password {
    min: usize,
    max: usize,
    c: char,
    s: String,
}

impl Password {
    pub fn check1(&self) -> bool {
        let c_count = self.s.chars().filter(|&c| c == self.c).count();
        c_count <= self.max && c_count >= self.min
    }

    pub fn check2(&self) -> bool {
        let a = self.s.chars().nth(self.min - 1).unwrap();
        let b = self.s.chars().nth(self.max - 1).unwrap();
        (a == self.c) ^ (b == self.c)
    }
}

impl FromStr for Password {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r"([0-9]+)-([0-9]+) ([a-z]): ([a-z]*)").unwrap();
        let c = re.captures_iter(s).next().unwrap();
        Ok(Self {
            min: c[1].parse().unwrap(),
            max: c[2].parse().unwrap(),
            c: c[3].parse().unwrap(),
            s: c[4].to_string(),
        })
    }
}

pub fn part1() {
    let data: Vec<Password> = read_file("./resources/2_1.txt").unwrap();
    let nb = data
        .iter()
        .filter_map(|d| if d.check1() { Some(d) } else { None })
        .count();

    println!("{:?}", nb);
}

pub fn part2() {
    let data: Vec<Password> = read_file("./resources/2_1.txt").unwrap();
    let nb = data
        .iter()
        .filter_map(|d| if d.check2() { Some(d) } else { None })
        .count();

    println!("{:?}", nb);
}
