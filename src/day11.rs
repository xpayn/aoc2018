use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt;

struct Cell {
    x: usize,
    y: usize,
}

impl Cell {
    fn rack_id(&self) -> usize {
        self.x + 10
    }

    fn power(&self, serial: usize) -> i8 {
        let total = (self.rack_id() * self.y + serial) * self.rack_id();
        let hundreds = if total < 100 {
            0
        } else {
            total / 100 - total / 1000 * 10
        } as i8;
        hundreds - 5
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<i8> {
    let serial: usize = input.trim().parse().unwrap();
    let mut grid: Vec<i8> = Vec::with_capacity(300 * 300);
    for j in 0..300 {
        for i in 0..300 {
            grid.push(Cell { x: i + 1, y: j + 1 }.power(serial));
        }
    }
    grid
}

fn solve_generic_grid(input: &[i8], dim: usize) -> (usize, usize, usize, i32) {
    let fold_size = 300 - dim + 1;
    let mut line_fold: Vec<i32> = vec![0; 300 * fold_size];
    let mut complete_fold: Vec<i32> = vec![0; 300 * fold_size];
    for y in 0..300 {
        input
            .windows(dim)
            .skip(y * 300)
            .take(fold_size)
            .enumerate()
            .for_each(|(x, arr)| {
                let i = x * 300 + y; //transposition
                line_fold[i] = arr.iter().map(|&i| i32::from(i)).sum();
            })
    }

    let (mut i_max_power, mut max_power) = (0, std::i32::MIN);
    for y in 0..fold_size {
        line_fold
            .windows(dim)
            .skip(y * 300)
            .take(fold_size)
            .enumerate()
            .for_each(|(x, arr)| {
                let i = x * fold_size + y; //back transposition
                let power = arr.iter().sum();
                complete_fold[i] = power;
                if power > max_power {
                    i_max_power = i;
                    max_power = power;
                }
            })
    }

    let x = i_max_power % fold_size + 1;
    let y = i_max_power / fold_size + 1;

    (x, y, dim, max_power)
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &[i8]) -> String {
    let (x, y, _, _) = solve_generic_grid(input, 3);
    format!("{},{}", x, y)
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &[i8]) -> String {
    let mut max_output = (0, 0, 0, std::i32::MIN);
    for dim in 1..301 {
        let output = solve_generic_grid(input, dim);
        if output.3 > max_output.3 {
            max_output = output;
        }
    }
    let (x, y, dim, _) = max_output;
    format!("{},{},{}", x, y, dim)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn power_levels() {
        assert_eq!(Cell { x: 3, y: 5 }.power(8), 4);
        assert_eq!(Cell { x: 122, y: 79 }.power(57), -5);
        assert_eq!(Cell { x: 217, y: 196 }.power(39), 0);
        assert_eq!(Cell { x: 101, y: 153 }.power(71), 4);
    }

    #[test]
    fn d11_part1() {
        assert_eq!(solve_part1(&input_generator("18")), "33,45");
        assert_eq!(solve_part1(&input_generator("42")), "21,61");
    }

    #[test]
    fn d11_part2() {
        assert_eq!(solve_part2(&input_generator("18")), "90,269,16");
        assert_eq!(solve_part2(&input_generator("42")), "232,251,12");
    }
}
