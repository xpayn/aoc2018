use std::collections::HashMap;
use std::iter::FromIterator;

use aoc_runner_derive::{aoc, aoc_generator};

type TransitionTable = HashMap<Vec<bool>, bool>;

#[derive(Clone, PartialEq, Eq)]
pub struct State {
    store: Vec<bool>,
    left_index: isize,
}

#[allow(dead_code)]
fn format_store(store: &[bool]) -> String {
    store
        .iter()
        .map(|&b| if b { '#' } else { '.' })
        .collect::<String>()
}

fn shrink_store(store: &[bool]) -> (isize, &[bool]) {
    let (alpha, _) = store.iter().enumerate().find(|(_, &b)| b).unwrap();
    let (omega, _) = store.iter().enumerate().rev().find(|(_, &b)| b).unwrap();
    (alpha as isize, &store[alpha..=omega])
}

impl State {
    fn outer_left(&self, transitions: &TransitionTable, offset: usize) -> bool {
        let mut left = vec![false; offset];
        left.extend(self.store.iter().take(5 - offset).cloned());
        *transitions.get(&left).unwrap_or(&false)
    }

    fn outer_right(&self, transitions: &TransitionTable, offset: usize) -> bool {
        let mut right = Vec::from_iter(
            self.store
                .iter()
                .skip(self.store.len() - 5 + offset)
                .take(5 - offset)
                .cloned(),
        );
        right.extend(vec![false; offset]);
        *transitions.get(&right).unwrap_or(&false)
    }

    fn next(self, transitions: &TransitionTable) -> State {
        let mut store: Vec<bool> = vec![3, 2, 1]
            .into_iter()
            .map(|offset| self.outer_left(transitions, offset))
            .collect();

        store.extend(
            self.store
                .windows(5)
                .map(|arr| *transitions.get(arr).unwrap_or(&false)),
        );

        store.extend((1..4).map(|offset| self.outer_right(transitions, offset)));

        let (delta, shrinked) = shrink_store(&store);

        State {
            store: shrinked.to_vec(),
            left_index: self.left_index - 1 + delta,
        }
    }

    fn sum_index(&self) -> isize {
        self.store
            .iter()
            .enumerate()
            .filter(|(_, &b)| b)
            .fold(0, |acc, x| acc + x.0 as isize + self.left_index)
    }
}

fn parse_seq(s: &str) -> Vec<bool> {
    s.chars()
        .map(|c| match c {
            '.' => false,
            '#' => true,
            _ => {
                println!("AAAAA {}", c);
                unreachable!()
            }
        })
        .collect()
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<(State, TransitionTable)> {
    let lines: Vec<&str> = input.lines().collect();
    let v: Vec<&str> = lines[0].trim().split(' ').collect();
    let initial_state = State {
        store: parse_seq(v[2]),
        left_index: 0,
    };
    let transitions = lines
        .iter()
        .skip(2)
        .map(|l| {
            let v: Vec<&str> = l.trim().split(" => ").collect();
            let pattern = parse_seq(v[0]);
            let outcome = parse_seq(v[1]).pop().unwrap();
            (pattern, outcome)
        })
        .collect();
    vec![(initial_state, transitions)]
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &[(State, TransitionTable)]) -> isize {
    let mut s = input[0].0.clone();

    for _ in 0..20 {
        s = s.next(&input[0].1);
    }

    s.sum_index()
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &[(State, TransitionTable)]) -> isize {
    let mut s = input[0].0.clone();
    let mut last = s.clone();
    let mut i = 0;
    loop {
        s = s.next(&input[0].1);
        if s.store == last.store {
            println!("{} {}", last.left_index, s.left_index);
            break;
        }
        i += 1;
        last = s.clone();
    }

    State {
        store: last.store,
        left_index: last.left_index + 50_000_000_000 - i,
    }
    .sum_index()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn d12_part1() {
        let input = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";
        assert_eq!(solve_part1(&input_generator(input)), 325);
    }
}
