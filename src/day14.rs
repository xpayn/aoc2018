use aoc_runner_derive::aoc;

fn parse_seq(input: &str) -> Vec<u8> {
    input
        .trim()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|i| i as u8)
        .collect()
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &str) -> String {
    let mut recipes = vec![3, 7];
    let n = input.trim().parse::<usize>().unwrap();
    let stop_at = n + 10;
    let mut elves_indexes = (0, 1);
    while recipes.len() < stop_at {
        let curr_0 = recipes[elves_indexes.0];
        let curr_1 = recipes[elves_indexes.1];
        let combine = format!("{}", curr_0 + curr_1);
        parse_seq(&combine)
            .into_iter()
            .for_each(|i| recipes.push(i));
        elves_indexes = (
            (elves_indexes.0 + 1 + curr_0 as usize) % recipes.len(),
            (elves_indexes.1 + 1 + curr_1 as usize) % recipes.len(),
        )
    }
    let mut buffer = String::with_capacity(10);
    recipes[n..stop_at]
        .iter()
        .for_each(|i| buffer.push_str(&format!("{}", i)));
    buffer
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &str) -> usize {
    let stop_seq = parse_seq(input);
    let mut recipes = vec![3, 7];
    let mut elves_indexes = (0, 1);
    while recipes.len() < stop_seq.len() + 1
        || stop_seq[..] != recipes[recipes.len() - stop_seq.len()..]
            && stop_seq[..] != recipes[recipes.len() - 1 - stop_seq.len()..recipes.len() - 1]
    {
        let curr_0 = recipes[elves_indexes.0];
        let curr_1 = recipes[elves_indexes.1];
        let combine = format!("{}", curr_0 + curr_1);
        parse_seq(&combine)
            .into_iter()
            .for_each(|i| recipes.push(i));
        elves_indexes = (
            (elves_indexes.0 + 1 + curr_0 as usize) % recipes.len(),
            (elves_indexes.1 + 1 + curr_1 as usize) % recipes.len(),
        );
    }
    let mut ret = recipes.len() - stop_seq.len();
    if stop_seq[..] == recipes[recipes.len() - 1 - stop_seq.len()..recipes.len() - 1] {
        ret -= 1;
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d14_part1() {
        assert_eq!(solve_part1("9"), "5158916779");
        assert_eq!(solve_part1("5"), "0124515891");
        assert_eq!(solve_part1("18"), "9251071085");
        assert_eq!(solve_part1("2018"), "5941429882");
    }

    #[test]
    fn d14_part2() {
        assert_eq!(solve_part2("51589"), 9);
        assert_eq!(solve_part2("01245"), 5);
        assert_eq!(solve_part2("92510"), 18);
        assert_eq!(solve_part2("59414"), 2018);
        assert_eq!(solve_part2("51071"), 20);
    }
}
