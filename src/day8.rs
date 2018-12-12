use aoc_runner_derive::{aoc, aoc_generator};

struct Node {
    children: Vec<Node>,
    metadata: Vec<u8>,
}

impl Node {
    fn sum_metadata(&self) -> usize {
        self.metadata.iter().map(|&d| d as usize).sum()
    }

    fn recursive_sum_metadata(&self) -> usize {
        self.children
            .iter()
            .map(|c| c.recursive_sum_metadata())
            .sum::<usize>()
            + self.sum_metadata()
    }

    fn value(&self) -> usize {
        if self.children.is_empty() {
            self.sum_metadata()
        } else {
            self.metadata
                .iter()
                .filter(|&&i| i > 0 && i <= self.children.len() as u8)
                .map(|&i| self.children[(i - 1) as usize].value())
                .sum()
        }
    }
}

fn build_tree<'a, I>(iter: &mut I) -> Node
where
    I: Iterator<Item = &'a u8>,
{
    let nb_children = *iter.next().unwrap();
    let nb_metadata = *iter.next().unwrap();
    let children = (0..nb_children).map(|_| build_tree(iter)).collect();
    let metadata = iter.take(nb_metadata as usize).cloned().collect();

    Node { children, metadata }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<u8> {
    input.split(' ').map(|i| i.parse().unwrap()).collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[u8]) -> usize {
    build_tree(&mut input.into_iter()).recursive_sum_metadata()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[u8]) -> usize {
    build_tree(&mut input.into_iter()).value()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn d8_part1() {
        let input = input_generator("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
        assert_eq!(solve_part1(&input), 138);
    }

    #[test]
    fn d8_part2() {
        let input = input_generator("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
        assert_eq!(solve_part2(&input), 66);
    }
}
