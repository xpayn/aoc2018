use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

use aoc_runner_derive::aoc;

#[derive(Clone)]
enum Warrior {
    Elf(i16),
    Goblin(i16),
}

impl fmt::Display for Warrior {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Warrior::Elf(_) => 'E',
                Warrior::Goblin(_) => 'G',
            }
        )
    }
}

impl fmt::Debug for Warrior {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Warrior::Elf(hp) => write!(f, "E({})", hp),
            Warrior::Goblin(hp) => write!(f, "G({})", hp),
        }
    }
}

impl Warrior {
    fn get_hit_points(&self) -> i16 {
        match *self {
            Warrior::Elf(p) => p,
            Warrior::Goblin(p) => p,
        }
    }

    fn is_dead(&self) -> bool {
        self.get_hit_points() <= 0
    }

    fn take_damage(&mut self, attack_power: i16) {
        match *self {
            Warrior::Elf(ref mut p) => *p -= attack_power,
            Warrior::Goblin(ref mut p) => *p -= attack_power,
        }
    }
}

#[derive(Clone)]
enum Tile {
    Wall,
    Void,
    Occupied(Warrior),
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Tile::Wall => write!(f, "#"),
            Tile::Void => write!(f, "."),
            Tile::Occupied(ref warrior) => warrior.fmt(f),
        }
    }
}

struct Position {
    x: usize,
    y: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Position {
    fn distance(&self, other: &Position) -> usize {
        ((self.x as isize - other.x as isize).abs() + (self.y as isize - other.y as isize).abs())
            as usize
    }

    fn north(&self) -> Position {
        Position {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn south(&self) -> Position {
        Position {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn west(&self) -> Position {
        Position {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn east(&self) -> Position {
        Position {
            x: self.x + 1,
            y: self.y,
        }
    }
}

#[derive(Clone)]
struct Cavern {
    width: usize,
    grid: Vec<Tile>,
    globlins_attack_power: i16,
    elves_attack_power: i16,
}

impl fmt::Display for Cavern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.grid
            .chunks(self.width)
            .map(|arr| {
                arr.iter()
                    .map(|e| e.fmt(f))
                    .collect::<fmt::Result>()
                    .and(writeln!(f))
            })
            .collect()
    }
}

impl fmt::Debug for Cavern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.grid
            .chunks(self.width)
            .map(|arr| {
                let mut acc = vec![];
                arr.iter()
                    .map(|e| {
                        if let Tile::Occupied(warrior) = e {
                            acc.push(format!("{:?}", warrior))
                        }
                        fmt::Display::fmt(e, f)
                    })
                    .collect::<fmt::Result>()
                    .and(writeln!(f, "  {}", acc.join(", ")))
            })
            .collect()
    }
}

#[derive(PartialEq, Eq)]
enum FightOutcome {
    ElvesAllDead,
    GoblinsAllDead,
    AnElfDied,
    Ongoing,
}

impl Cavern {
    fn get_goblins_indexes(&self) -> Vec<usize> {
        self.grid
            .iter()
            .enumerate()
            .filter_map(|(i, tile)| match tile {
                Tile::Occupied(Warrior::Goblin(_)) => Some(i),
                _ => None,
            })
            .collect()
    }

    fn get_elves_indexes(&self) -> Vec<usize> {
        self.grid
            .iter()
            .enumerate()
            .filter_map(|(i, tile)| match tile {
                Tile::Occupied(Warrior::Elf(_)) => Some(i),
                _ => None,
            })
            .collect()
    }

    fn get_warriors_indexes(&self) -> Vec<usize> {
        self.grid
            .iter()
            .enumerate()
            .filter_map(|(i, tile)| match tile {
                Tile::Occupied(_) => Some(i),
                _ => None,
            })
            .collect()
    }

    fn advance(&mut self, stop_if_elf_dies: bool) -> FightOutcome {
        for mut curr_i in self.get_warriors_indexes() {
            if let Tile::Occupied(ref mut current) = self.grid[curr_i] {
                let is_elf = match *current {
                    Warrior::Elf(_) => true,
                    _ => false,
                };

                let (attack_power, mut hostiles) = if is_elf {
                    (self.elves_attack_power, self.get_goblins_indexes())
                } else {
                    (self.globlins_attack_power, self.get_elves_indexes())
                };

                if hostiles.is_empty() {
                    return if is_elf {
                        FightOutcome::GoblinsAllDead
                    } else {
                        FightOutcome::ElvesAllDead
                    };
                }

                // sort by distance, then by reading order
                hostiles.sort_by(|&a, &b| {
                    match self
                        .naive_distance(curr_i, a)
                        .cmp(&self.naive_distance(curr_i, b))
                    {
                        std::cmp::Ordering::Equal => a.cmp(&b),
                        rest => rest,
                    }
                });

                if self.naive_distance(curr_i, hostiles[0]) != 1 {
                    // let's move
                    let in_range = hostiles
                        .iter()
                        .flat_map(|&i| self.get_tiles_in_range_of(i))
                        .collect::<HashSet<usize>>();

                    if in_range.is_empty() {
                        continue;
                    }

                    let mut reachable = in_range
                        .iter()
                        .filter_map(|&tile| self.path(curr_i, tile))
                        .collect::<Vec<Vec<usize>>>();

                    if reachable.is_empty() {
                        continue;
                    }

                    reachable.sort_by(|a, b| match a.len().cmp(&b.len()) {
                        std::cmp::Ordering::Equal => a.cmp(&b),
                        rest => rest,
                    });

                    self.grid[reachable[0][0]] = self.grid[curr_i].clone();
                    self.grid[curr_i] = Tile::Void;
                    curr_i = reachable[0][0];
                }

                let mut adjacents = hostiles
                    .iter()
                    .cloned()
                    .filter(|&i| self.naive_distance(curr_i, i) == 1)
                    .collect::<Vec<usize>>();

                if !adjacents.is_empty() {
                    // let's move
                    // attack!!!!
                    adjacents.sort_by(|&a, &b| {
                        if let Tile::Occupied(ref mut hostile_a) = self.grid[a].clone() {
                            if let Tile::Occupied(ref mut hostile_b) = self.grid[b].clone() {
                                return match hostile_a
                                    .get_hit_points()
                                    .cmp(&hostile_b.get_hit_points())
                                {
                                    std::cmp::Ordering::Equal => a.cmp(&b),
                                    rest => rest,
                                };
                            }
                        }
                        unreachable!()
                    });

                    if let Tile::Occupied(ref mut target) = self.grid[adjacents[0]] {
                        let is_elf = match target {
                            Warrior::Elf(_) => true,
                            _ => false,
                        };
                        target.take_damage(attack_power);
                        if target.is_dead() {
                            self.grid[adjacents[0]] = Tile::Void;
                            if is_elf && stop_if_elf_dies {
                                return FightOutcome::AnElfDied;
                            }
                        }
                    }
                }
            }
        }
        FightOutcome::Ongoing
    }

    fn get_tiles_in_range_of(&self, index: usize) -> Vec<usize> {
        let p = self.index_to_position(index);
        let mut v = vec![p.north(), p.south(), p.east(), p.west()]
            .iter()
            .filter_map(|p| {
                let i = self.position_to_index(p);
                match self.grid[i] {
                    Tile::Void => Some(i),
                    _ => None,
                }
            })
            .collect::<Vec<usize>>();
        v.sort();
        v
    }

    fn position_to_index(&self, pos: &Position) -> usize {
        pos.x + pos.y * self.width
    }

    fn index_to_position(&self, index: usize) -> Position {
        Position {
            x: index % self.width,
            y: index / self.width,
        }
    }

    fn naive_distance(&self, a: usize, b: usize) -> usize {
        self.index_to_position(a)
            .distance(&self.index_to_position(b))
    }

    fn path(&self, root: usize, goal: usize) -> Option<Vec<usize>> {
        let mut visited: HashSet<usize> = HashSet::new();
        let mut path_builder: HashMap<usize, usize> = HashMap::new();
        let mut fifo: VecDeque<usize> = VecDeque::new();
        fifo.push_back(root);

        while let Some(subtree_root) = fifo.pop_front() {
            if subtree_root == goal {
                let mut state = goal;
                let mut path = vec![];
                while let Some(new_state) = path_builder.get(&state) {
                    path.push(state);
                    state = *new_state;
                }
                path.reverse();
                return Some(path);
            }

            for child in self.get_tiles_in_range_of(subtree_root) {
                if visited.contains(&child) {
                    continue;
                }

                path_builder.entry(child).or_insert_with(|| {
                    fifo.push_back(child);
                    subtree_root
                });
            }

            visited.insert(subtree_root);
        }

        None
    }

    fn sum_hit_points(&self) -> usize {
        self.get_warriors_indexes()
            .into_iter()
            .map(|i| match self.grid[i] {
                Tile::Occupied(ref current) => current.get_hit_points() as usize,
                _ => unreachable!(),
            })
            .sum()
    }

    fn fight_til_the_end(&mut self) -> usize {
        let mut i = 0;
        //println!("Initially:\n{:?}", self);
        while self.advance(false) == FightOutcome::Ongoing {
            i += 1;
            //println!("After {} rounds\n{:?}", i, self);
        }
        i * self.sum_hit_points()
    }

    fn fight_til_first_elf_dies(&mut self) -> (FightOutcome, usize) {
        let mut i = 0;
        //println!("Initially:\n{:?}", self);
        let mut outcome = self.advance(true);
        while outcome == FightOutcome::Ongoing {
            i += 1;
            outcome = self.advance(true);
            //println!("After {} rounds\n{:?}", i, self);
        }
        (outcome, i * self.sum_hit_points())
    }
}

fn parse_input(input: &str, elves_attack_power: i16) -> Cavern {
    Cavern {
        width: input.find('\n').unwrap(),
        grid: input
            .lines()
            .flat_map(|l| {
                l.chars().map(|c| {
                    use self::Tile::*;
                    match c {
                        '#' => Wall,
                        '.' => Void,
                        'G' => Occupied(Warrior::Goblin(200)),
                        'E' => Occupied(Warrior::Elf(200)),
                        _ => unreachable!(),
                    }
                })
            })
            .collect(),
        globlins_attack_power: 3,
        elves_attack_power,
    }
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut cavern = parse_input(input, 3);
    cavern.fight_til_the_end()
}

fn give_elves_some_steroids(orig: &Cavern, power: i16) -> (FightOutcome, usize) {
    let mut cavern = orig.clone();
    cavern.elves_attack_power = power;
    cavern.fight_til_first_elf_dies()
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &str) -> usize {
    let (mut min, mut max) = (4, 25);
    let orig = &parse_input(input, min);

    let (outcome, mut score) = give_elves_some_steroids(orig, min);
    if outcome == FightOutcome::GoblinsAllDead {
        return score;
    }

    let (mut outcome, _) = give_elves_some_steroids(orig, max);
    while outcome != FightOutcome::GoblinsAllDead {
        max *= 2;
        outcome = give_elves_some_steroids(orig, max).0;
    }

    while max - min > 1 {
        let mid = (max + min) / 2;
        let (outcome, score_tmp) = give_elves_some_steroids(orig, mid);

        if outcome == FightOutcome::GoblinsAllDead {
            max = mid;
            score = score_tmp;
        } else {
            min = mid;
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d15_part1() {
        let tests = vec![
            (
                "#######\n#.E..G#\n#.#####\n#G#####\n#######",
                "#######\n#E..G.#\n#G#####\n#.#####\n#######\n",
            ),
            ("####\n#GG#\n#.E#\n####", "####\n#.G#\n#GE#\n####\n"),
            (
                "########\n#..E..G#\n#G######\n########",
                "########\n#GE..G.#\n#.######\n########\n",
            ),
        ];

        for t in tests {
            let mut c = parse_input(t.0, 3);
            c.advance(false);
            assert_eq!(format!("{}", c), t.1);
        }

        let tests_full = vec![
            (
                "#######\n#.G...#\n#...EG#\n#.#.#G#\n#..G#E#\n#.....#\n#######",
                27730),
            (
                "#######\n#G..#E#\n#E#E.E#\n#G.##.#\n#...#E#\n#...E.#\n#######",
                36334,
            ),
            (
                "#######\n#E..EG#\n#.#G.E#\n#E.##E#\n#G..#.#\n#..E#.#\n#######",
                39514,
            ),
            (
                "#######\n#E.G#.#\n#.#G..#\n#G.#.G#\n#G..#.#\n#...E.#\n#######",
                27755,
            ),
            (
                "#######\n#.E...#\n#.#..G#\n#.###.#\n#E#G#G#\n#...#G#\n#######",
                28944,
            ),
            (
                "#########\n#G......#\n#.E.#...#\n#..##..G#\n#...##..#\n#...#...#\n#.G...G.#\n#.....G.#\n#########",
                18740,
            ),
        ];

        for t in tests_full {
            assert_eq!(solve_part1(t.0), t.1);
        }
    }

    #[test]
    fn d15_part2() {}
}
