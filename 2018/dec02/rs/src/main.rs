extern crate clap;

use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use clap::{Arg, App};
use std::collections::HashMap;

fn exactly_23(box_id: &str) -> (bool, bool) {
    let mut counts = HashMap::new();
    for c in box_id.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    let (mut exactly_2, mut exactly_3) = (false, false);
    for count in counts.values() {
        match count {
            2 => exactly_2 = true,
            3 => exactly_3 = true,
            _ => {},
        }
        if exactly_2 && exactly_3 {
            break
        }
    }
    (exactly_2, exactly_3)
}

fn are_correct(box_id1: &str, box_id2: &str) -> Option<String> {
    let mut common = Vec::new();
    let mut diff = false;
    for (c1, c2) in box_id1.chars().zip(box_id2.chars()) {
        if c1 != c2 {
            if diff {
                return None
            }
            diff = true;
        } else {
            common.push(c1.to_string());
        }
    }
    println!("{:?}", common);
    Some(common.join(""))
}

fn part1(input: &str) -> Result<i32, Box<Error>> {
    let f = File::open(input)?;
    let buf = BufReader::new(f);
    let (mut total_2, mut total_3) = (0, 0);
    for line in buf.lines() {
        let (exactly_2, exactly_3) = exactly_23(line?.as_str());
        total_2 += exactly_2 as i32;
        total_3 += exactly_3 as i32;
    }
    Ok(total_2 * total_3)
}

fn part2(input: &str) -> Result<String, Box<Error>> {
    let mut box_ids = Vec::new();
    let f = File::open(input)?;
    let buf = BufReader::new(f);
    for line in buf.lines() {
        box_ids.push(line?);
    }

    for i in 0..box_ids.len() {
        for j in i+1..box_ids.len() {
            if let Some(common) = are_correct(&box_ids[i], &box_ids[j]) {
                return Ok(common);
            }
        }
    }

    Ok("".to_string())
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
