use itertools::Itertools;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

pub struct Validator;

impl Validator {
    pub fn byr(data: &str) -> bool {
        if let Ok(parsed) = data.parse::<u32>() {
            1920 <= parsed && parsed <= 2002
        } else {
            false
        }
    }

    pub fn iyr(data: &str) -> bool {
        if let Ok(parsed) = data.parse::<u32>() {
            2010 <= parsed && parsed <= 2020
        } else {
            false
        }
    }
}

type PassportRaw = HashMap<String, String>;

pub fn is_valid(passport: &PassportRaw) -> bool {
    let fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    fields.iter().all(|&f| passport.keys().any(|k| k == f))
}

pub fn read_file(p: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let f = File::open(p)?;
    let buf_reader = BufReader::new(f);
    let mut res = Vec::new();
    let mut tmp = Vec::new();

    for line in buf_reader.lines() {
        match line {
            Ok(l) if !l.is_empty() => tmp.push(l.clone()),
            _ => {
                res.push(tmp.clone());
                tmp.clear()
            }
        }
    }

    Ok(res)
}

pub fn create_passports(data: Vec<Vec<String>>) -> Vec<PassportRaw> {
    data.iter()
        .map(|fields| {
            let mut p = HashMap::new();
            fields.iter().for_each(|f| {
                f.split(' ').for_each(|ff| {
                    let position = ff.find(':').unwrap();
                    let (k, v) = ff.split_at(position);
                    p.insert(k.to_string(), v.to_string());
                })
            });
            p
        })
        .collect()
}

pub fn part1() {
    let data = read_file("./resources/4_1.txt").unwrap();
    let passports = create_passports(data);
    let cnt = passports.iter().filter(|p| is_valid(p)).count();
    println!("{:?}", cnt);
}
