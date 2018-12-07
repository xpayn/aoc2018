use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

// 852,570
struct Position {
    x: usize,
    y: usize,
}

impl FromStr for Position {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();

        Ok(Self {
            x: coords[0].parse::<usize>()?,
            y: coords[1].parse::<usize>()?,
        })
    }
}

// 13x16
struct Dimension {
    w: usize,
    h: usize,
}

impl FromStr for Dimension {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split('x').collect();

        Ok(Self {
            w: coords[0].parse::<usize>()?,
            h: coords[1].parse::<usize>()?,
        })
    }
}

// #1229 @ 852,570: 13x16
pub struct Claim {
    id: u16,
    pos: Position,
    dim: Dimension,
}

impl FromStr for Claim {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let claim: Vec<&str> = s.trim_matches(|p| p == '#').split(" @ ").collect();
        let dsf: Vec<&str> = claim[1].split(": ").collect();

        Ok(Self {
            id: claim[0].parse::<u16>()?,
            pos: dsf[0].parse::<Position>()?,
            dim: dsf[1].parse::<Dimension>()?,
        })
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Claim> {
    input.lines().map(|l| l.trim().parse().unwrap()).collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Claim]) -> usize {
    _solve_part1(input, 1000, 1000)
}

fn _solve_part1(input: &[Claim], width: usize, height: usize) -> usize {
    // overlap stored lini by line
    let mut fabric: Vec<u8> = vec![0; width * height];
    input.iter().for_each(|claim| {
        let offset = claim.pos.y * width + claim.pos.x;
        for i in 0..claim.dim.h {
            for j in 0..claim.dim.w {
                let p: usize = i * height + j;
                fabric[offset + p] += 1;
            }
        }
    });

    fabric.into_iter().filter(|x| *x > 1).count()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Claim]) -> u16 {
    _solve_part2(input, 1000, 1000)
}

fn _solve_part2(input: &[Claim], width: usize, height: usize) -> u16 {
    let mut claims: HashMap<u16, bool> = HashMap::new();
    // overlap stored lini by line
    let mut fabric: Vec<u16> = vec![0; width * height];
    input.iter().for_each(|claim| {
        claims.insert(claim.id, false);
        let offset = claim.pos.y * width + claim.pos.x;
        for i in 0..claim.dim.h {
            for j in 0..claim.dim.w {
                let p: usize = i * height + j;
                let v = fabric[offset + p];
                if v > 0 {
                    if let Some(c) = claims.get_mut(&v) {
                        *c = true; //tainted
                    }
                    if let Some(c) = claims.get_mut(&claim.id) {
                        *c = true; //tainted
                    }
                }
                fabric[offset + p] = claim.id;
            }
        }
    });

    claims.into_iter().find(|(_, c)| !*c).unwrap().0
}
