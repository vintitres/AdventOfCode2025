use std::{collections::BTreeSet, ops::Bound};

use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let ranges = lines
        .by_ref()
        .take_while(|line| !line.trim().is_empty())
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .collect_vec();
    lines
        .map(|i| i.parse::<u64>().unwrap())
        .filter(|&ingredient| {
            ranges
                .iter()
                .any(|&(start, end)| start <= ingredient && ingredient <= end)
        })
        .count()
}

pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .take_while(|line| !line.trim().is_empty())
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .fold(
            BTreeSet::<(u64, u64)>::new(),
            |mut ranges, (mut start, mut end)| {
                println!("Range: {} {}", start, end);
                let mut cursor = ranges.lower_bound_mut(Bound::Included(&(start, start)));
                if let Some(&(prev_start, prev_end)) = cursor.peek_prev() {
                    if prev_end >= start {
                        start = prev_start;
                        cursor.remove_prev();
                    }
                }
                while let Some(&(next_start, next_end)) = cursor.peek_next() {
                    if end < next_start {
                        break;
                    }
                    end = next_end;
                    cursor.remove_next();
                }
                let _ = cursor.insert_after((start, end));
                println!("Ranges: {:?}", ranges);
                ranges
            },
        )
        .into_iter()
        .map(|(start, end)| end - start + 1)
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2025/day5.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 720);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 5782);
    }
}
