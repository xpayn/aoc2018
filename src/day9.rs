use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<usize> {
    let data: Vec<&str> = input.split(' ').collect();
    let nb_players = data[0].parse::<usize>().unwrap();
    let last_marble = data[6].parse::<usize>().unwrap();
    vec![nb_players, last_marble + 1]
}

#[allow(dead_code)]
fn print_circle(circle: &[usize], current: usize) {
    for c in circle.iter().take(current) {
        print!(" {} ", c);
    }

    print!("({})", circle[current]);

    for c in circle.iter().skip(current + 1) {
        print!(" {} ", c);
    }
    println!()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    let nb_players = input[0];
    let nb_marbles = input[1];
    let mut circle = Vec::with_capacity(nb_marbles);
    let mut players = vec![0; nb_players];
    let mut current = 0;
    circle.push(0);
    //println!("[-] (0)");
    for m in 1..nb_marbles {
        //print!("[{}] ", m % nb_players + 1);
        let len = circle.len();
        let mut pos;
        if m % 23 == 0 {
            pos = (len + current - 7) % len;
            players[m % nb_players] += m + circle.remove(pos);
        } else {
            pos = (current + 2) % len;
            if pos == 0 {
                pos = len;
            }
            circle.insert(pos, m);
        }
        current = pos;
        //print_circle(&circle, current);
    }
    players.into_iter().max().unwrap()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    let nb_players = input[0];
    let nb_marbles = 10 * input[1];
    solve_part1(&[nb_players, nb_marbles])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn d9_part1() {
        let v = [
            ("9 players; last marble is worth 25 points", 32),
            ("10 players; last marble is worth 1618 points", 8317),
            ("13 players; last marble is worth 7999 points", 146373),
            ("17 players; last marble is worth 1104 points", 2764),
            ("21 players; last marble is worth 6111 points", 54718),
            ("30 players; last marble is worth 5807 points", 37305),
        ];

        for t in v.iter() {
            assert_eq!(solve_part1(&input_generator(t.0)), t.1);
        }
    }
}
