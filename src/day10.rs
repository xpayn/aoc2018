use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::{max, min, Ord, Ordering, PartialOrd};
use std::fmt;
use std::ops::Range;

#[derive(Eq)]
pub struct Point {
    x: i32,
    y: i32,
    velocity: (i32, i32),
}

type RawPoint = ((i32, i32), (i32, i32));

impl Point {
    fn advance(&mut self) {
        self.x += self.velocity.0 as i32;
        self.y += self.velocity.1 as i32;
    }

    fn rewind(&mut self) {
        self.x -= self.velocity.0 as i32;
        self.y -= self.velocity.1 as i32;
    }
}

impl From<RawPoint> for Point {
    fn from(i: RawPoint) -> Self {
        Point {
            x: (i.0).0,
            y: (i.0).1,
            velocity: i.1,
        }
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        self.y.cmp(&other.y).then_with(|| self.x.cmp(&other.x))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub struct Grid {
    points: Vec<Point>,
    x_range: Range<i32>,
    y_range: Range<i32>,
}

impl Grid {
    fn advance(&mut self) {
        self.step_with(|p| p.advance());
    }

    fn rewind(&mut self) {
        self.step_with(|p| p.rewind());
    }

    fn step_with<F>(&mut self, f: F)
    where
        F: Fn(&mut Point),
    {
        self.x_range = std::i32::MAX..std::i32::MIN;
        self.y_range = std::i32::MAX..std::i32::MIN;
        for p in self.points.iter_mut() {
            f(p);
            self.x_range = min(self.x_range.start, p.x)..max(self.x_range.end, p.x);
            self.y_range = min(self.y_range.start, p.y)..max(self.y_range.end, p.y);
        }

        self.points.sort()
    }

    fn width(&self) -> i32 {
        (self.x_range.end - self.x_range.start + 1)
    }

    fn height(&self) -> i32 {
        (self.y_range.end - self.y_range.start + 1)
    }

    fn converge(&mut self) -> usize {
        let mut ret = 0;
        let mut last_h = self.height();
        while self.height() <= last_h {
            last_h = self.height();
            self.advance();
            ret += 1;
        }
        self.rewind();
        ret - 1
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x_offset = self.x_range.start;
        let y_offset = self.y_range.start;
        let w = self.width();

        let mut y_last = 0;
        let mut line: Vec<u8> = vec![b'.'; w as usize];
        for p in self.points.iter() {
            let x = p.x - x_offset;
            let y = p.y - y_offset;
            if y != y_last {
                writeln!(f, "{}", String::from_utf8(line).unwrap())?;
                line = vec![b'.'; w as usize];
                let nb_blank_lines = y - y_last - 1;
                for _ in 0..nb_blank_lines {
                    writeln!(f, "{}", ".".repeat(w as usize))?;
                }
            }
            line[x as usize] = b'#';
            y_last = y;
        }
        writeln!(f, "{}", String::from_utf8(line).unwrap())
    }
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<RawPoint> {
    let mut x_range = (0, 0);
    let mut y_range = (0, 0);
    let mut v: Vec<RawPoint> = input
        .lines()
        .map(|l| {
            let v: Vec<&str> = l.trim().split(", ").collect();

            let x: i32 = v[0]
                .trim()
                .split('<')
                .last()
                .unwrap()
                .trim()
                .parse()
                .unwrap();

            let v_y: i32 = v[2]
                .trim()
                .split('>')
                .next()
                .unwrap()
                .trim()
                .parse()
                .unwrap();

            let v: Vec<&str> = v[1].trim().split("> velocity=<").collect();
            let y: i32 = v[0].trim().parse().unwrap();

            let v_x: i32 = v[1].trim().parse().unwrap();
            x_range = (min(x_range.0, x), max(x_range.1, x));
            y_range = (min(y_range.0, y), max(y_range.1, y));
            ((x, y), (v_x, v_y))
        })
        .collect();
    v.push((x_range, y_range));
    v
}

fn build_grid(input: &[RawPoint]) -> Grid {
    let mut points: Vec<Point> = input
        .iter()
        .take(input.len() - 1)
        .map(|&i| Point::from(i))
        .collect();
    points.sort();

    let (x_range, y_range) = input.last().unwrap();
    Grid {
        points,
        x_range: x_range.0..x_range.1,
        y_range: y_range.0..y_range.1,
    }
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[RawPoint]) -> String {
    let mut g = build_grid(input);
    g.converge();
    format!("\n{}", g)
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[RawPoint]) -> usize {
    let mut g = build_grid(input);
    g.converge()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn d10_part1() {
        let input = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";
        let res = "\n#...#..###
#...#...#.
#...#...#.
#####...#.
#...#...#.
#...#...#.
#...#...#.
#...#..###\n";
        assert_eq!(solve_part1(&input_generator(input)), res);
    }
}
