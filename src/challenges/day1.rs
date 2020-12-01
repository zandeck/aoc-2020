use crate::challenges::utils;
use itertools::Itertools;
use std::collections::HashSet;

fn f(data: &[i32], combi: usize) -> i32 {
    data.iter()
        .combinations(combi)
        .find_map(|e| {
            if e.iter().copied().sum::<i32>() == 2020 {
                Some(e.iter().copied().product())
            } else {
                None
            }
        })
        .unwrap()
}

fn g(data: &[i32]) -> i32 {
    Combination::new(data)
        .find_map(|(a, b)| if a + b == 2020 { Some(a * b) } else { None })
        .unwrap()
}

pub fn part1_1(data: &[i32]) -> i32 {
    //let data: Vec<i32> = utils::read_file("./resources/1_1.txt").unwrap();
    f(data, 2)
    //println!("{:?}", f(data, 2));
}

pub fn part1_2(data: &[i32]) -> i32 {
    //let data: Vec<i32> = utils::read_file("./resources/1_1.txt").unwrap();
    g(data)
    //println!("{:?}", f(data, 2));
}

pub fn part1_3(data: &[i32]) -> i32 {
    //let data: Vec<i32> = utils::read_file("./resources/1_1.txt").unwrap();
    let s: HashSet<i32> = data.iter().cloned().collect();
    data.iter()
        .find_map(|&e| {
            if s.contains(&(2020 - e)) {
                Some((2020 - e) * e)
            } else {
                None
            }
        })
        .unwrap()
}

pub fn part2_2(data: &[i32]) -> i32 {
    // let data: Vec<i32> = utils::read_file("./resources/1_1.txt").unwrap();

    data.iter()
        .find_map(|a| {
            data.iter().find_map(|b| {
                data.iter().find_map(|c| {
                    if a + b + c == 2020 {
                        Some(a * b * c)
                    } else {
                        None
                    }
                })
            })
        })
        .unwrap()
}

pub fn part2_1(data: &[i32]) -> i32 {
    //let data: Vec<i32> = utils::read_file("./resources/1_1.txt").unwrap();
    data.iter()
        .combinations(3)
        .find_map(|e| {
            if e[0] + e[1] + e[2] == 2020 {
                Some(e[0] * e[1] * e[2])
            } else {
                None
            }
        })
        .unwrap()
}

pub fn part2_3(data: &[i32]) -> i32 {
    //let data: Vec<i32> = utils::read_file("./resources/1_1.txt").unwrap();
    Combination3::new(data)
        .find_map(|(a, b, c)| {
            if a + b + c == 2020 {
                Some(a * b * c)
            } else {
                None
            }
        })
        .unwrap()
}
pub struct Combination<'a> {
    data: &'a [i32],
    it1: usize,
    it2: usize,
    end: bool,
}

impl<'a> Combination<'a> {
    pub fn new(data: &'a [i32]) -> Self {
        Combination {
            data,
            it1: 0,
            it2: 0,
            end: false,
        }
    }
}

impl<'a> Iterator for Combination<'a> {
    type Item = (&'a i32, &'a i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.it2 + 1 >= self.data.len() {
            if self.it1 == self.data.len() - 2 {
                None
            } else {
                self.it1 += 1;
                self.it2 = self.it1 + 1;
                Some((&self.data[self.it1], &self.data[self.it2]))
            }
        } else {
            self.it2 += 1;
            Some((&self.data[self.it1], &self.data[self.it2]))
        }
    }
}
pub struct Combination3<'a> {
    data: &'a [i32],
    it1: usize,
    it2: usize,
    it3: usize,
}

impl<'a> Combination3<'a> {
    pub fn new(data: &'a [i32]) -> Self {
        Combination3 {
            data,
            it1: 0,
            it2: 0,
            it3: 0,
        }
    }
}

impl<'a> Iterator for Combination3<'a> {
    type Item = (&'a i32, &'a i32, &'a i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.it3 + 1 >= self.data.len() {
            if self.it2 + 1 >= self.data.len() - 1 {
                if self.it1 + 1 >= self.data.len() - 2 {
                    None
                } else {
                    self.it1 += 1;
                    self.it2 = self.it1 + 1;
                    self.it3 = self.it1 + 2;
                    Some((
                        &self.data[self.it1],
                        &self.data[self.it2],
                        &self.data[self.it3],
                    ))
                }
            } else {
                self.it2 += 1;
                self.it3 = self.it2 + 1;
                Some((
                    &self.data[self.it1],
                    &self.data[self.it2],
                    &self.data[self.it3],
                ))
            }
        } else {
            self.it3 += 1;
            Some((
                &self.data[self.it1],
                &self.data[self.it2],
                &self.data[self.it3],
            ))
        }
    }
}
