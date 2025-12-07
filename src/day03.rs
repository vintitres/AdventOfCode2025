use std::{collections::BTreeSet, fmt::Pointer};

use itertools::Itertools;

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .rev()
                .scan(None, |seen_max, x| {
                    if let Some(_seen_max) = *seen_max {
                        let v = x * 10 + _seen_max;
                        *seen_max = Some(std::cmp::max(_seen_max, x));
                        Some(v)
                    } else {
                        *seen_max = Some(x);
                        Some(0)
                    }
                })
                .max()
                .unwrap()
        })
        .sum()
}

fn max_joltage(bank: &[u32], length: usize) -> u64 {
    if length == 0 {
        return 0;
    }
    let max = bank[..bank.len() - length + 1].iter().max().unwrap();
    let index = bank.iter().position(|x| x == max).unwrap();
    *max as u64 * 10_u64.pow(length as u32 - 1) + max_joltage(&bank[index + 1..], length - 1)
}

pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            max_joltage(
                &line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec(),
                12,
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2025/day3.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 17109);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 169347417057382);
    }
}
