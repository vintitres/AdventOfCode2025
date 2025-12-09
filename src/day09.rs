use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let (x, y) = line
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap();
            (x, y)
        })
        .combinations(2)
        .map(|combin| {
            let a = combin[0];
            let b = combin[1];
            let (ax, ay) = a;
            let (bx, by) = b;
            ((ax - bx).abs() + 1) * ((ay - by).abs() + 1)
        })
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> i64 {
    let red = input
        .lines()
        .map(|line| {
            let (x, y) = line
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap();
            (x, y)
        })
        .collect_vec();
    let x_mapping = red
        .iter()
        .map(|(x, _)| *x)
        .sorted()
        .unique()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect::<HashMap<i64, usize>>();
    let y_mapping = red
        .iter()
        .map(|(_, y)| *y)
        .sorted()
        .unique()
        .enumerate()
        .map(|(i, y)| (y, i))
        .collect::<HashMap<i64, usize>>();

    let (last_x, last_y) = red[red.len() - 1];
    let mut last_x_index = x_mapping[&last_x];
    let mut last_y_index = y_mapping[&last_y];
    let mut grid = vec![vec![false; y_mapping.len()]; x_mapping.len()];
    for (x, y) in &red {
        let x_index = x_mapping[&x];
        let y_index = y_mapping[&y];
        for i in min(x_index, last_x_index)..=max(x_index, last_x_index) {
            for j in min(y_index, last_y_index)..=max(y_index, last_y_index) {
                grid[i][j] = true;
            }
        }
        last_x_index = x_index;
        last_y_index = y_index;
    }
    let mut queue = VecDeque::new();
    queue.push_back((last_x_index - 1, last_y_index - 1)); // hardcoded
    while let Some((x, y)) = queue.pop_front() {
        if grid[x][y] {
            continue;
        }
        grid[x][y] = true;
        for (i, j) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if !grid[i][j] {
                queue.push_back((i, j));
            }
        }
    }
    red.iter()
        .combinations(2)
        .map(|combin| {
            let a = combin[0];
            let b = combin[1];
            let (ax, ay) = a;
            let (bx, by) = b;
            let ax_index = x_mapping[&ax];
            let bx_index = x_mapping[&bx];
            let ay_index = y_mapping[&ay];
            let by_index = y_mapping[&by];
            for i in min(ax_index, bx_index)..=max(ax_index, bx_index) {
                for j in min(ay_index, by_index)..=max(ay_index, by_index) {
                    if !grid[i][j] {
                        return 0;
                    }
                }
            }
            ((ax - bx).abs() + 1) * ((ay - by).abs() + 1)
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2025/day9.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 4782896435);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 1540060480);
    }
}
