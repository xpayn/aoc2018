use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> String {
    reduce(input, None)
}

pub fn reduce(input: &str, filter: Option<char>) -> String {
    let len = input.len();
    let mut reduced = String::with_capacity(len);
    let mut iter = input.chars().peekable();
    while let Some(c) = iter.next() {
        let c_l = c.to_ascii_lowercase();
        if Some(c_l) == filter {
            continue;
        }
        if let Some(next) = iter.peek() {
            if c_l == next.to_ascii_lowercase() && c != *next {
                iter.next();
                continue;
            }
        }
        reduced.push(c);
    }

    if reduced.len() != len {
        reduce(&reduced, filter)
    } else {
        reduced
    }
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> usize {
    input.len()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut res: usize = std::usize::MAX;
    for c in (b'a'..=b'z').map(char::from) {
        res = std::cmp::min(res, reduce(input, Some(c)).len());
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        assert_eq!(
            solve_part1(&input_generator("dabAcCaCBAcCcaDA")),
            "dabCBAcaDA".len()
        );
    }
}
