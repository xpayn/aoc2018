use std::collections::{HashMap, HashSet, VecDeque};

use aoc_runner_derive::aoc;

#[derive(Clone)]
enum Warrior {
    Elf(i16),
    Goblin(i16),
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

    fn take_damage(&mut self) {
        match *self {
            Warrior::Elf(ref mut p) => *p -= 3,
            Warrior::Goblin(ref mut p) => *p -= 3,
        }
    }
}

#[derive(Clone)]
enum Tile {
    Wall,
    Void,
    Occupied(Warrior),
}

struct Position {
    x: usize,
    y: usize,
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

struct Cavern {
    width: usize,
    grid: Vec<Tile>,
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

    fn advance(&mut self) -> bool {
        for curr_i in self.get_warriors_indexes() {
            if let Tile::Occupied(ref mut current) = self.grid[curr_i] {
                let mut hostiles = match *current {
                    Warrior::Elf(_) => self.get_goblins_indexes(),
                    Warrior::Goblin(_) => self.get_elves_indexes(),
                };

                if hostiles.is_empty() {
                    return false;
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

                if self.naive_distance(curr_i, hostiles[0]) == 1 {
                    // attack!!!!
                    if let Tile::Occupied(ref mut target) = self.grid[hostiles[0]] {
                        target.take_damage();
                        if target.is_dead() {
                            self.grid[hostiles[0]] = Tile::Void
                        }
                    }
                } else {
                    let in_range = hostiles
                        .into_iter()
                        .flat_map(|i| self.get_tiles_in_range_of(i))
                        .collect::<HashSet<usize>>();

                    if in_range.is_empty() {
                        continue;
                    }

                    let mut reachable = in_range
                        .into_iter()
                        .filter_map(|tile| self.path(curr_i, tile))
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
                }
            }
        }
        true
    }

    fn get_tiles_in_range_of(&self, index: usize) -> Vec<usize> {
        let p = self.index_to_position(index);
        vec![p.north(), p.south(), p.east(), p.west()]
            .iter()
            .filter_map(|p| {
                let i = self.position_to_index(p);
                match self.grid[i] {
                    Tile::Void => Some(i),
                    _ => None,
                }
            })
            .collect()
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
                path.push(root);
                path.reverse();
                return Some(path);
            }

            for child in self.get_tiles_in_range_of(subtree_root) {
                if visited.contains(&child) {
                    continue;
                }

                if !path_builder.contains_key(&child) {
                    path_builder.insert(child, subtree_root);
                    fifo.push_back(child);
                }
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
            .inspect(|i| println!("aaa {}", i))
            .sum()
    }
}

fn parse_input(input: &str) -> Cavern {
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
    }
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut cavern = parse_input(input);
    let mut i = 0;
    while cavern.advance() {
        i += 1;
    }
    i * cavern.sum_hit_points()
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &str) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d15_part1() {
        let tests = vec![
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

        for t in tests {
            assert_eq!(solve_part1(t.0), t.1);
        }
    }

    #[test]
    fn d15_part2() {}
}
