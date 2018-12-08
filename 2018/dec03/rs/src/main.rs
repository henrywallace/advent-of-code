extern crate clap;

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use clap::{Arg, App};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use regex::Regex;

#[derive(Debug)]
struct Claim {
    id: u32,
    offset: (u32, u32),
    size: (u32, u32),
}

impl FromStr for Claim {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            // e.g. "#1 @ 509,796: 18x15"
            static ref RE: Regex = Regex::new(r"(?x)
                \#(?P<id>\d+)\s+@\s+
                (?P<x>\d+),(?P<y>\d+):\s+
                (?P<w>\d+)x(?P<h>\d+)
            ").unwrap();
        }
        match RE.captures(s) {
            Some(caps) => {
                Ok(Claim{
                    id: caps["id"].parse()?,
                    offset: (caps["x"].parse()?, caps["y"].parse()?),
                    size: (caps["w"].parse()?, caps["h"].parse()?),
                })
            },
            None => Err(Box::<Error>::from(format!("failed to create claim from: {:?}", s))),
        }
    }
}

impl Claim {
    fn update_fabric(&self, fabric: &mut Fabric) {
        for i in self.offset.0..(self.offset.0+self.size.0) {
            for j in self.offset.1..(self.offset.1+self.size.1) {
                fabric.entry((i, j))
                    .or_insert(vec![])
                    .push(self.id);
            }
        }
    }
}

#[cfg(test)]
// Test! woot! https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html.
mod tests {
    use super::*;

    #[test]
    fn test_fabric_one_simple() {
        let claim: Claim = "#1 @ 1,2: 1x2".parse().unwrap();
        let mut actual = Fabric::new();
        claim.update_fabric(&mut actual);
        let expect: Fabric = [
            ((1, 2), vec![1]),
            ((1, 3), vec![1]),
        ].iter().cloned().collect();
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_fabric_two_overlap() {
        let claim1: Claim = "#1 @ 0,0: 2x1".parse().unwrap();
        let claim2: Claim = "#2 @ 1,0: 2x1".parse().unwrap();
        let mut actual = Fabric::new();
        claim1.update_fabric(&mut actual);
        claim2.update_fabric(&mut actual);
        let expect: Fabric = [
            ((0, 0), vec![1]), ((1, 0), vec![1, 2]), ((2, 0), vec![2]),
        ].iter().cloned().collect();
        assert_eq!(expect, actual);
    }
}

type Fabric = HashMap<(u32, u32), Vec<u32>>;

fn part1(input: &str) -> Result<(), Box<Error>> {
    let mut fabric = Fabric::new();
    let f = File::open(input)?;
    let buf = BufReader::new(f);
    for line in buf.lines() {
        let claim: Claim = line?.parse()?;
        claim.update_fabric(&mut fabric);
    }
    let mut total = 0;
    for v in fabric.values() {
        if v.len() > 1 {
            total += 1;
        }
    }
    println!("{}", total);
    Ok(())
}

fn part2(input: &str) -> Result<(), Box<Error>> {
    let mut fabric = Fabric::new();
    let f = File::open(input)?;
    let buf = BufReader::new(f);
    for line in buf.lines() {
        let claim: Claim = line?.parse()?;
        claim.update_fabric(&mut fabric);
    }
    let mut alone: HashSet<u32> = HashSet::new();
    let mut not_alone: HashSet<u32> = HashSet::new();
    for v in fabric.values() {
        if v.len() == 1 {
            alone.insert(v[0]);
        } else {
            for id in v.iter() {
                not_alone.insert(*id);
            }
        }
    }
    println!("{:?}", alone.difference(&not_alone));
    Ok(())
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
          "1" => part1(input).unwrap(),
          "2" => part2(input).unwrap(),
          _ => unreachable!(),
    }
}
