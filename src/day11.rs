use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use itertools::{GroupingMap, Itertools};

pub fn part1(input: &str) -> usize {
    let graph: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|line| {
            let (from, tos) = line.split_once(": ").unwrap();
            (from, tos.split(' ').collect_vec())
        })
        .collect();
    let mut queue = vec!["you"];
    let mut ways = 0;
    while let Some(node) = queue.pop() {
        if node == "out" {
            ways += 1;
            continue;
        }
        for neighbor in graph.get(node).unwrap_or(&vec![]) {
            queue.push(neighbor);
        }
    }
    ways
}

fn dfs_ways(
    graph: &HashMap<&str, Vec<&str>>,
    from: &str,
    to: &str,
    mem: &mut HashMap<(&str, &str), usize>,
) {
}

pub fn part2(input: &str) -> usize {
    /*
     * 1 0 0
     * 0 1 0
     * 0 0 1
     *
     *
     */
    let graph: HashMap<(&str, &str), usize> = input
        .lines()
        .map(|line| {
            let (from, tos) = line.split_once(": ").unwrap();
            tos.split(' ').map(|to| ((from, to), 1))
        })
        .flatten()
        .collect();

    let mut combinations = HashMap::new();
    let mut queue = vec!["fft"];
    let mut ways = 0;
    while let Some((node, seen)) = queue.pop() {
        if node == "out" {
            ways += 1;
            continue;
        }
        for &neighbor in graph.get(node).unwrap_or(&vec![]) {
            if seen.contains(neighbor) {
                continue;
            }
            let mut new_seen = seen.clone();
            new_seen.insert(neighbor);
            queue.push((neighbor, new_seen));
        }
    }
    ways
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2025/day11.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 494);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 5782);
    }
}
