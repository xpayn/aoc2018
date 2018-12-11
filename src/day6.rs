use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, PartialEq, Debug)]
pub enum Area {
    Finite(usize),
    Infinite,
}

#[derive(Clone)]
pub struct Point {
    x: usize,
    y: usize,
    area: Area,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(", ").collect();

        Ok(Self {
            x: coords[0].parse::<usize>()?,
            y: coords[1].parse::<usize>()?,
            area: Area::Finite(0),
        })
    }
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Point> {
    input.lines().map(|l| l.trim().parse().unwrap()).collect()
}

fn get_grid_dim(input: &[Point]) -> (usize, usize, usize, usize) {
    let ((min_x, min_y), (max_x, max_y)) =
        input
            .iter()
            .fold(((std::usize::MAX, std::usize::MAX), (0, 0)), |acc, p| {
                let ((mut min_x, mut min_y), (mut max_x, mut max_y)) = acc;
                if p.x < min_x {
                    min_x = p.x;
                } else if p.x > max_x {
                    max_x = p.x;
                }

                if p.y < min_y {
                    min_y = p.y;
                } else if p.y > max_y {
                    max_y = p.y;
                }

                ((min_x, min_y), (max_x, max_y))
            });
    let width = max_x - min_x;
    let height = max_y - min_y;

    (width, height, min_x, min_y)
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[Point]) -> usize {
    let (width, height, offset_x, offset_y) = get_grid_dim(input);

    let mut grid = vec![(std::usize::MAX, std::usize::MAX); width * height];
    for (i, p) in input.iter().enumerate() {
        let px = (p.x - offset_x) as i64;
        let py = (p.y - offset_y) as i64;
        for (j, e) in grid.iter_mut().enumerate() {
            let gx = (j % width) as i64;
            let gy = (j / width) as i64;
            let d = ((px - gx).abs() + (py - gy).abs()) as usize;
            if e.1 > d {
                e.0 = i as usize;
                e.1 = d;
            } else if e.1 == d {
                e.0 = std::usize::MAX;
            }
        }
    }
    let mut h = HashMap::new();
    for (i, e) in grid.iter_mut().enumerate() {
        if e.0 == std::usize::MAX {
            continue;
        }
        let p = h.entry(e.0).or_insert_with(|| input[e.0].clone());
        if i % width == 0 || i < width || i % width == width - 1 || i > width * (height - 1) {
            p.area = Area::Infinite;
        } else if let Area::Finite(cpt) = p.area {
            p.area = Area::Finite(cpt + 1);
        }
    }

    let a = h
        .into_iter()
        .max_by(|(_, a), (_, b)| match a.area {
            Area::Infinite => std::cmp::Ordering::Less,
            Area::Finite(a) => match b.area {
                Area::Infinite => std::cmp::Ordering::Greater,
                Area::Finite(b) => a.cmp(&b),
            },
        })
        .unwrap()
        .1
        .area;

    if let Area::Finite(a) = a {
        a
    } else {
        0
    }
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[Point]) -> usize {
    let (width, height, offset_x, offset_y) = get_grid_dim(input);
    let mut grid = vec![0; width * height];
    for (i, e) in grid.iter_mut().enumerate() {
        let gx = (i % width) as i64;
        let gy = (i / width) as i64;
        for p in input {
            let px = (p.x - offset_x) as i64;
            let py = (p.y - offset_y) as i64;
            let d = ((px - gx).abs() + (py - gy).abs()) as usize;
            *e += d;
            if *e >= 10000 {
                break;
            }
        }
    }

    grid.into_iter().filter(|x| *x < 10000).count()
}
