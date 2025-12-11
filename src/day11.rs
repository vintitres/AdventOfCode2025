use std::collections::HashMap;

use itertools::Itertools;

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

fn dfs_ways<'a>(
    graph: &'a HashMap<&str, Vec<&str>>,
    from: &'a str,
    to: &'a str,
    mem: &mut HashMap<(&'a str, &'a str), u64>,
) -> u64 {
    if from == to {
        return 1;
    }
    if let Some(&ways) = mem.get(&(from, to)) {
        return ways;
    }
    let mut ways = 0;
    for neighbor in graph.get(from).unwrap_or(&vec![]) {
        ways += dfs_ways(graph, neighbor, to, mem);
    }
    mem.insert((from, to), ways);
    ways
}

pub fn part2(input: &str) -> u64 {
    let graph: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|line| {
            let (from, tos) = line.split_once(": ").unwrap();
            (from, tos.split(' ').collect_vec())
        })
        .collect();
    let mut mem = HashMap::new();
    let ways_fft_out = dfs_ways(&graph, "fft", "out", &mut mem);
    let ways_dac_out = dfs_ways(&graph, "dac", "out", &mut mem);
    let ways_fft_dac = dfs_ways(&graph, "fft", "dac", &mut mem);
    let ways_dac_fft = dfs_ways(&graph, "dac", "fft", &mut mem);
    let ways_svr_fft = dfs_ways(&graph, "svr", "fft", &mut mem);
    let ways_svr_dac = dfs_ways(&graph, "svr", "dac", &mut mem);
    ways_svr_dac * ways_dac_fft * ways_fft_out + ways_svr_fft * ways_fft_dac * ways_dac_out
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 296006754704850);
    }
}
