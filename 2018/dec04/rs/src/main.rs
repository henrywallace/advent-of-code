extern crate clap;
extern crate chrono;

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use clap::{Arg, App};
use chrono::{Duration, NaiveDateTime};
use std::str::FromStr;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Record {
    timestamp: chrono::NaiveDateTime,
    event: Event,
}

type GuardID = u32;

#[derive(Debug)]
enum Event {
    Begin { guard_id: GuardID },
    WakesUp,
    FallsAsleep,
}

impl FromStr for Record {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            // [1518-06-12 00:00] Guard #3359 begins shift
            // [1518-10-08 00:19] wakes up
            // [1518-06-07 00:26] falls asleep
            static ref RE: Regex = Regex::new(r"(?x)
                \[(?P<timestamp>.+)\]\s+
                (?:Guard\ \#(?P<id>\d+)\ begins\ shift|(?P<repose>.+))
            ").unwrap();
        }
        let caps = match RE.captures(s) {
            None => return Err(Box::<Error>::from(format!("failed to parse Record from: {:?}", s))),
            Some(caps) => caps,
        };
        let ts = NaiveDateTime::parse_from_str(&caps["timestamp"], "%Y-%m-%d %H:%M")?;
        let event: Event =
            if let Some(id) = caps.name("id") {
                Event::Begin{guard_id: id.as_str().parse()?}
            } else if let Some(repose) = caps.name("repose") {
                match repose.as_str() {
                    "falls asleep" => Event::FallsAsleep,
                    "wakes up" => Event::WakesUp,
                    _ => unreachable!("oh no!"),
                }
            } else {
                return Err(Box::<Error>::from(format!("failed to parse Event from: {:?}", s)))
            };

        Ok(Record{
            timestamp: ts,
            event: event,
        })
    }
}

fn total_asleep(records: Vec<Record>) -> Result<HashMap<GuardID, Duration>, Box<Error>> {
    let mut totals = HashMap::new();
    if records.is_empty() {
        return Ok(totals)
    }
    let mut it = records.iter();
    let head = it.next().unwrap(); // TODO: handle empty records
    let mut curr_id: GuardID;
    let mut last_asleep: Option<chrono::NaiveDateTime> = None;
    match head.event {
        Event::Begin{guard_id} => curr_id = guard_id,
        _ => return Err(Box::<Error>::from(format!("unexpected first record: {:?}", head))),
    }

    for rec in it {
        match rec.event {
            Event::Begin{guard_id} => {
                if let Some(ts) = last_asleep {
                    let dur = rec.timestamp.signed_duration_since(ts);
                    let prev = *totals.entry(curr_id).or_insert(Duration::zero());
                    totals.insert(curr_id, prev + dur);
                };
                curr_id = guard_id;
            },
            Event::FallsAsleep => last_asleep = Some(rec.timestamp),
            Event::WakesUp => {
                if let Some(ts) = last_asleep {
                    let dur = rec.timestamp.signed_duration_since(ts);
                    let prev = *totals.entry(curr_id).or_insert(Duration::zero());
                    totals.insert(curr_id, prev + dur);
                };
                last_asleep = None;
            }
        }
    }
    Ok(totals)
}

fn part1(input: &str) -> Result<(), Box<Error>> {
    let f = File::open(input)?;
    let buf = BufReader::new(f);
    let mut records = vec![];
    for line in buf.lines() {
        let rec: Record = line?.parse()?;
        records.push(rec);
    }
    records.sort_by(|rec1, rec2| rec1.timestamp.cmp(&rec2.timestamp));
    let totals = total_asleep(records)?;
    let mut top: GuardID = 0;
    let mut most_sleepy = Duration::zero();
    for (id, dur) in totals {
        if dur > most_sleepy {
            top = id;
            most_sleepy = dur;
        }
    }
    println!("Guard {} slept the most at {}min", top, most_sleepy.num_minutes());
    Ok(())
}

fn main() {
    let matches = App::new("dec04")
        .arg(Arg::with_name("input").required(true))
        .arg(Arg::with_name("part").long("part").required(true)
             .takes_value(true)
             .possible_values(&["1", "2"]))
        .get_matches();

    let input = matches.value_of("input").unwrap();
    match matches.value_of("part").unwrap() {
          "1" => part1(input).unwrap(),
          // "2" => part2(input).unwrap(),
          _ => unreachable!(),
    }
}
