use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

pub fn read_file<T>(p: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let f = File::open(p)?;
    let buf_reader = BufReader::new(f);
    Ok(buf_reader
        .lines()
        .map(|l| FromStr::from_str(&l.unwrap()).unwrap())
        .collect())
}
