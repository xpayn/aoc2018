use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
pub struct Step {
    id: char,
    allows: Vec<char>,
    blocked_by: Vec<char>,
    has_dependencies: bool,
}

impl Step {
    fn get_time(&self, penalty: usize) -> usize {
        penalty + 1 + self.id as usize - 'A' as usize
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<(char, char)> {
    input
        .lines()
        .map(|l| {
            let v: Vec<&str> = l.trim().split(" must be finished before step ").collect();
            (v[0].chars().last().unwrap(), v[1].chars().next().unwrap())
        })
        .collect()
}

fn build_graph(input: &[(char, char)]) -> HashMap<char, Step> {
    let mut h: HashMap<char, Step> = HashMap::new();
    input.iter().for_each(|(dependency, depends)| {
        let a = h.entry(*dependency).or_insert(Step {
            id: *dependency,
            allows: vec![],
            blocked_by: vec![],
            has_dependencies: false,
        });
        a.allows.push(*depends);
        a.allows.sort();

        let d = h.entry(*depends).or_insert(Step {
            id: *depends,
            allows: vec![],
            blocked_by: vec![],
            has_dependencies: false,
        });
        d.has_dependencies = true;
        d.blocked_by.push(*dependency);
    });
    h
}

fn find_standalone_tasks(tasks: &HashMap<char, Step>) -> Vec<Step> {
    let mut ready: Vec<Step> = tasks
        .iter()
        .filter(|(_, s)| !s.has_dependencies)
        .map(|(_, s)| s.clone())
        .collect();
    ready.sort_by(|a, b| b.id.cmp(&a.id));
    ready
}

fn unlock_tasks(completed: &Step, tasks: &HashMap<char, Step>, ready: &mut Vec<Step>) {
    'outer: for s in completed.allows.iter().map(|id| &tasks[&id]) {
        for blocker in s.blocked_by.iter() {
            if tasks.contains_key(blocker) {
                continue 'outer;
            }
        }
        ready.push(s.clone())
    }
    ready.sort_by(|a, b| b.id.cmp(&a.id));
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[(char, char)]) -> String {
    let mut tasks = build_graph(input);
    let mut done = "".to_owned();
    let mut ready = find_standalone_tasks(&tasks);

    while !ready.is_empty() {
        let next = ready.pop().unwrap();
        done.push(next.id);
        tasks.remove(&next.id);
        unlock_tasks(&next, &tasks, &mut ready);
    }

    done
}

#[allow(dead_code)]
fn print_state(time: usize, doing: &[(i64, Option<Step>)], done: &str) {
    print!("{}\t", time);
    for (_, current) in doing.iter() {
        let w = match current {
            None => '.',
            Some(t) => t.id,
        };
        print!("{}\t", w);
    }

    println!("{}", done);
}

pub fn generic_solve_part2(input: &[(char, char)], nb_worker: usize, penalty: usize) -> usize {
    let mut done = "".to_owned();
    let mut doing: Vec<(i64, Option<Step>)> = vec![(0, None); nb_worker];
    let mut tasks = build_graph(input);
    let nb_tasks = tasks.len();
    let mut ready: Vec<Step> = find_standalone_tasks(&tasks);

    let mut time = 0;

    while done.len() != nb_tasks {
        for (eta, current) in doing.iter_mut().filter(|(_, t)| t.is_some()) {
            // advance tasks completion
            *eta -= 1;
            // check if tasks are complete and free workers if it is the case
            if *eta == 0 {
                let completed = current.as_ref().unwrap();
                done.push(completed.id);
                tasks.remove(&completed.id);
                unlock_tasks(completed, &tasks, &mut ready);
                *current = None;
            }
        }
        // assign tasks to worker if there's tasks ready and workers available
        for (eta, current) in doing.iter_mut().filter(|(_, t)| t.is_none()) {
            if let Some(task) = ready.pop() {
                *eta = task.get_time(penalty) as i64;
                *current = Some(task);
            }
        }

        //print_state(time, &doing, &done);
        time += 1;
    }

    time - 1
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[(char, char)]) -> usize {
    generic_solve_part2(input, 5, 60)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part2() {
        let input = "Step C must be finished before step A can begin.\nStep C must be finished before step F can begin.\nStep A must be finished before step B can begin.\nStep A must be finished before step D can begin.\nStep B must be finished before step E can begin.\nStep D must be finished before step E can begin.\nStep F must be finished before step E can begin.";
        assert_eq!(generic_solve_part2(&input_generator(input), 2, 0), 15);
    }
}
