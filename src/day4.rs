use std::collections::HashMap;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use bitvec::{bitvec, BigEndian, BitVec};
use chrono::{NaiveDateTime, Timelike};
use failure::{format_err, Error};
use itertools::Itertools;
use time::Duration;

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq)]
enum Event {
    BeginsShift,
    WakesUp,
    FallsAsleep,
}

impl FromStr for Event {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "begins shift" => Ok(Event::BeginsShift),
            "falls asleep" => Ok(Event::FallsAsleep),
            "wakes up" => Ok(Event::WakesUp),
            _ => Err(format_err!("unknown event")),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct RawRecord {
    timestamp: NaiveDateTime,
    event: Event,
    guard: Option<u16>,
}

impl FromStr for RawRecord {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<&str> = s.split(' ').collect();

        let mut dt = NaiveDateTime::parse_from_str(
            format!("{} {}", fields[0], fields[1]).as_ref(),
            "[%Y-%m-%d %H:%M]",
        )?;

        let (g, e);
        if fields[2] == "Guard" {
            g = Some(fields[3].trim_matches(|p| p == '#').parse::<u16>()?);
            e = Event::BeginsShift;
            if dt.time().hour() == 23 {
                dt += Duration::days(1);
            }
            dt = dt.with_hour(0).unwrap().with_minute(0).unwrap();
        } else {
            g = None;
            e = format!("{} {}", fields[2], fields[3]).parse::<Event>()?;
        }

        Ok(RawRecord {
            timestamp: dt,
            guard: g,
            event: e,
        })
    }
}

pub struct Record {
    guard: u16,
    state: BitVec,
}

impl From<Vec<RawRecord>> for Record {
    fn from(mut v: Vec<RawRecord>) -> Self {
        v.sort();
        let g = v[0].guard.unwrap();
        let mut s = bitvec![0; 60];

        for r in v.into_iter().skip(1) {
            let value = r.event == Event::FallsAsleep;
            let m = r.timestamp.time().minute() as usize;
            for i in m..60 {
                s.set(i, value);
            }
        }

        Record { guard: g, state: s }
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Record> {
    input
        .lines()
        .map(|l| {
            let r = l.trim().parse::<RawRecord>().unwrap();
            (r.timestamp.date(), r)
        })
        .into_group_map()
        .into_iter()
        .map(|(_, v_r)| Record::from(v_r))
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Record]) -> usize {
    let mut h: HashMap<u16, Vec<usize>> = HashMap::new();
    for r in input {
        let cnt = h.entry(r.guard).or_insert_with(|| vec![0; 60]);
        r.state
            .iter()
            .zip(cnt.iter_mut())
            .for_each(|(s, c)| *c += s as usize);
    }
    let g = h
        .into_iter()
        .max_by(|(_, x), (_, y)| {
            let x_s = x.iter().sum::<usize>();
            let y_s = y.iter().sum::<usize>();
            x_s.cmp(&y_s)
        })
        .unwrap();
    let m: usize =
        g.1.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap()
            .0;
    (g.0 as usize) * m
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Record]) -> usize {
    let mut h: HashMap<u16, Vec<usize>> = HashMap::new();
    for r in input {
        let cnt = h.entry(r.guard).or_insert_with(|| vec![0; 60]);
        r.state
            .iter()
            .zip(cnt.iter_mut())
            .for_each(|(s, c)| *c += s as usize);
    }
    let g = h
        .into_iter()
        .map(|(id, v)| {
            let max = v
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.cmp(b))
                .unwrap();
            (id as usize, max.0, *max.1)
        })
        .max_by(|(_, _, a), (_, _, b)| a.cmp(b))
        .unwrap();
    println!("{:?}", g);
    g.0 * g.1
}
