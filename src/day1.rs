use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input.lines().map(|l| l.trim().parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i64]) -> i64 {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i64]) -> i64 {
    let r =
        input
            .iter()
            .cycle()
            .try_fold((0, HashSet::new()), |mut acc: (i64, HashSet<i64>), x| {
                acc.1.insert(acc.0);
                acc.0 += *x;
                if acc.1.contains(&acc.0) {
                    Err(acc)
                } else {
                    Ok(acc)
                }
            });
    match r {
        Err(i) => i.0,
        _ => std::i64::MAX,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part2() {
        assert_eq!(solve_part2(&vec![1, -1]), 0);
        assert_eq!(solve_part2(&vec![3, 3, 4, -2, -4]), 10);
        assert_eq!(solve_part2(&vec![-6, 3, 8, 5, -6]), 5);
        assert_eq!(solve_part2(&vec![7, 7, -2, -7, -4]), 14);
    }
}
