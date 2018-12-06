use std::collections::HashMap;
use itertools::Itertools;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|l| l.trim().to_owned()).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[String]) -> u64 {
    let (mut counter2, mut counter3) = (0, 0);

    input.iter().for_each(|s| {
        let mut h = HashMap::new();
        let (mut hit2, mut hit3) = (false, false);
        s.chars().for_each(|c| {
            let cnt = h.entry(c).or_insert(0);
            *cnt += 1;
        });
        for (_, cnt) in h {
            if cnt == 3 {
                hit3 = true;
            } else if cnt == 2 {
                hit2 = true;
            }
            if hit2 && hit3 {
                break;
            }
        }
        counter2 += hit2 as u64;
        counter3 += hit3 as u64;
    });

    counter2*counter3
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[String]) -> String {
    let r = input.iter().combinations(2).try_for_each(|v| {
        let (i, j) = (v[0], v[1]);
        let mut diff = false;
        let r = i.chars().zip(j.chars()).try_fold("".to_owned(), |mut acc, (c_i, c_j)| {
            if c_i == c_j {
                acc.push(c_i);
                Ok(acc)
            } else if diff {
                Err(())
            } else {
                diff = true;
                Ok(acc)
            }
        });
        match r {
            Ok(s) => Err(s),
            Err(()) => Ok(())
        }
    });

    if let Err(s) = r {
        return s;
    }

    panic!("you failed!")
}