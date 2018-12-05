extern crate clap;

use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use clap::{Arg, App};
use std::collections::HashSet;


fn part1(input: &str) -> Result<i32, Box<Error>> {
    let f = File::open(input)?;
    let buf = BufReader::new(f);
    let mut total = 0;
    for line in buf.lines() {
        total += line?.parse::<i32>()?
    }
    Ok(total)
}

fn part2(input: &str) -> Result<i32, Box<Error>> {
    let mut freqs = Vec::new();
    let f = File::open(input)?;
    let buf = BufReader::new(f);
    for line in buf.lines() {
        freqs.push(line?.parse::<i32>()?);
    }

    let mut seen = HashSet::new();
    seen.insert(0);

    let mut total = 0;
    loop {
        for freq in freqs.iter() {
          total += freq;
          if seen.contains(&total) {
              return Ok(total);
          }
          seen.insert(total);
        }
    }
}

fn main() {
    let matches = App::new("dec01")
        .arg(Arg::with_name("input").required(true))
        .arg(Arg::with_name("part").long("part").required(true)
             .takes_value(true)
             .possible_values(&["1", "2"]))
        .get_matches();

    let input = matches.value_of("input").unwrap();
    match matches.value_of("part").unwrap() {
          "1" => println!("{}", part1(input).unwrap()),
          "2" => println!("{}", part2(input).unwrap()),
          _ => unreachable!(),
    }
}
