use std::cmp::{Ord, Ordering, PartialOrd};
use std::fmt;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Clone)]
enum Orientation {
    Up,    // ^
    Down,  // v
    Left,  // <
    Right, // >
}

impl Orientation {
    fn is_vertical(&self) -> bool {
        use self::Orientation::*;
        match self {
            Up | Down => true,
            _ => false,
        }
    }

    fn is_horizontal(&self) -> bool {
        use self::Orientation::*;
        match self {
            Left | Right => true,
            _ => false,
        }
    }

    fn matches_track(&self, track: &Track) -> bool {
        match *track {
            Track::Vertical => self.is_vertical(),
            Track::Horizontal => self.is_horizontal(),
            _ => false,
        }
    }
}

#[derive(Clone)]
enum Track {
    Vertical,     // |
    Horizontal,   // -
    Intersection, // +
    Turn(char),   // / or \
    Cart(usize),  // ^, v, < or >
}

enum Crossing {
    Left,
    Right,
    Straight,
}

static CROSSING_SEQ: &'static [Crossing] = &[Crossing::Left, Crossing::Straight, Crossing::Right];

#[derive(Clone)]
pub struct Cart {
    direction: Orientation,
    crossing_index: usize,
    position: (usize, usize),
    id: usize,
}

impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> Ordering {
        let (x_self, y_self) = self.position;
        let (x_other, y_other) = &other.position;

        y_self.cmp(&y_other).then_with(|| x_self.cmp(x_other))
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cart {
    fn eq(&self, other: &Cart) -> bool {
        self.position == other.position
    }
}

impl Eq for Cart {}

impl Cart {
    fn new(c: char, x: usize, y: usize, id: usize) -> Self {
        use self::Orientation::*;

        let direction = match c {
            '<' => Left,
            '^' => Up,
            '>' => Right,
            'v' => Down,
            _ => unreachable!(),
        };
        Self {
            direction,
            crossing_index: 0,
            position: (x, y),
            id,
        }
    }

    fn turn(&mut self, c: char) {
        use self::Orientation::*;
        match c {
            '/' => {
                self.direction = match self.direction {
                    Up => Right,
                    Left => Down,
                    Down => Left,
                    Right => Up,
                }
            }
            '\\' => {
                self.direction = match self.direction {
                    Up => Left,
                    Left => Up,
                    Down => Right,
                    Right => Down,
                }
            }
            _ => unreachable!(),
        };
    }

    fn choose_direction(&mut self) {
        use self::Orientation::*;
        self.direction = match CROSSING_SEQ[self.crossing_index] {
            Crossing::Right => match self.direction {
                Up => Right,
                Left => Up,
                Down => Left,
                Right => Down,
            },
            Crossing::Left => match self.direction {
                Up => Left,
                Left => Down,
                Down => Right,
                Right => Up,
            },
            Crossing::Straight => self.direction.clone(),
        };

        self.crossing_index = (self.crossing_index + 1) % 3;
    }

    fn advance(&mut self, grid: &TrackGrid, others: &[Cart]) -> Option<((usize, usize), usize)> {
        use self::Orientation::*;
        use self::Track::*;

        let (mut x, mut y) = self.position;
        match self.direction {
            Up => y -= 1,
            Down => y += 1,
            Left => x -= 1,
            Right => x += 1,
        };
        let track = grid.get_track_at(x, y, others);
        match track {
            Cart(id) => return Some(((x, y), id)),
            Vertical | Horizontal => {
                if !self.direction.matches_track(&track) {
                    unreachable!()
                }
            }
            Turn(c) => self.turn(c),
            Intersection => self.choose_direction(),
        }
        self.position = (x, y);
        None
    }
}

pub struct TrackGrid {
    grid: Vec<Option<Track>>,
    width: usize,
}

#[allow(dead_code)]
fn print_grid(grid: &TrackGrid, carts: &[Cart]) {
    let mut buffer = format!("{}", grid).into_bytes();

    carts.iter().for_each(|c| {
        buffer[c.position.0 + c.position.1 * (grid.width + 1)] = match c.direction {
            Orientation::Right => '>',
            Orientation::Left => '<',
            Orientation::Up => '^',
            Orientation::Down => 'v',
        } as u8;
    });
    println!("{}", String::from_utf8(buffer).unwrap());
}

impl TrackGrid {
    fn advance(&self, carts: &mut Vec<Cart>, stop_at_first_crash: bool) -> Option<(usize, usize)> {
        let mut crashed_carts = vec![];
        for i in 0..carts.len() {
            if crashed_carts.contains(&carts[i].id) {
                continue;
            }
            let mut others = vec![];
            others.extend(carts[0..i].iter().cloned());
            others.extend(carts[i + 1..carts.len()].iter().cloned());
            let curr = &mut carts[i];
            if let Some((coord, id)) = curr.advance(self, &others) {
                if stop_at_first_crash {
                    return Some(coord);
                }
                crashed_carts.push(curr.id);
                crashed_carts.push(id);
            }
        }
        /*print_grid(self, carts);
        use std::thread;
        let one_s = std::time::Duration::from_millis(1000);
        thread::sleep(one_s);*/

        crashed_carts.into_iter().for_each(|c_id| {
            carts.remove(carts.iter().position(|c| c.id == c_id).unwrap());
        });

        if carts.len() == 1 {
            return Some(carts[0].position);
        }
        carts.sort();
        None
    }

    fn advance_until_crash(&self, carts: &mut Vec<Cart>) -> (usize, usize) {
        loop {
            if let Some(c) = self.advance(carts, true) {
                return c;
            }
        }
    }

    fn advance_until_last(&self, carts: &mut Vec<Cart>) -> (usize, usize) {
        let mut i = 0;
        loop {
            i += 1;
            if let Some(c) = self.advance(carts, false) {
                println!("AAAA {}", i);
                return c;
            }
        }
    }

    fn get_track_at(&self, x: usize, y: usize, carts: &[Cart]) -> Track {
        if let Some(cart) = carts.iter().find(|c| c.position == (x, y)) {
            Track::Cart(cart.id)
        } else {
            self.grid[y * self.width + x]
                .clone()
                .expect("out of track!!!!")
        }
    }
}

impl fmt::Display for TrackGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = vec![String::with_capacity(self.width); self.grid.len() / self.width];
        self.grid.iter().enumerate().for_each(|(i, t)| {
            use self::Track::*;
            buffer[i / self.width].push(if let Some(t) = t {
                match t {
                    Vertical => '|',
                    Horizontal => '-',
                    Intersection => '+',
                    Turn(c) => *c,
                    _ => unreachable!(),
                }
            } else {
                ' '
            });
        });
        buffer.iter().map(|l| writeln!(f, "{}", l)).collect()
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<(TrackGrid, Vec<Cart>)> {
    let mut v = vec![];
    let mut grid: Vec<Option<Track>> = vec![];
    let width = input.find('\n').unwrap();
    let mut carts = vec![];

    let mut last_char = ' ';
    grid.extend(input.lines().enumerate().flat_map(|(y, l)| {
        l.chars()
            .enumerate()
            .map(|(x, c)| {
                let res = match c {
                    ' ' => None,
                    '|' => Some(Track::Vertical),
                    '-' => Some(Track::Horizontal),
                    '+' => Some(Track::Intersection),
                    '/' | '\\' => Some(Track::Turn(c)),
                    '<' | '^' | '>' | 'v' => {
                        carts.push(Cart::new(c, x, y, x + y * width));
                        match last_char {
                            '-' | '+' => Some(Track::Horizontal),
                            _ => Some(Track::Vertical),
                        }
                    }
                    _ => unreachable!(),
                };
                last_char = c;
                res
            })
            .collect::<Vec<Option<Track>>>()
    }));
    v.push((TrackGrid { grid, width }, carts));
    v
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &[(TrackGrid, Vec<Cart>)]) -> String {
    let grid = &input[0].0;
    let mut carts = input[0].1.clone();
    let crash = grid.advance_until_crash(&mut carts);
    format!("{},{}", crash.0, crash.1)
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &[(TrackGrid, Vec<Cart>)]) -> String {
    let grid = &input[0].0;
    let mut carts = input[0].1.clone();
    let crash = grid.advance_until_last(&mut carts);
    format!("{},{}", crash.0, crash.1)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn crash() {
        assert_eq!(solve_part1(&input_generator(">-<\n")), "1,0");
        assert_eq!(solve_part1(&input_generator(">>-\n")), "1,0"); // not sure it should crash
        assert_eq!(solve_part1(&input_generator("><\n")), "1,0");
        assert_eq!(solve_part1(&input_generator("v\n+<\n")), "0,1");
    }

    #[test]
    fn d13_part1() {
        let input = r#"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   "#;
        assert_eq!(solve_part1(&input_generator(input)), "7,3");
    }

    #[test]
    fn d13_part2() {
        let input = r#"/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/"#;
        assert_eq!(solve_part2(&input_generator(input)), "6,4");
    }
}
