use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> usize {
    let mut beams = HashSet::new();
    let mut splits = 0;
    input.lines().for_each(|line| {
        if !beams.is_empty() {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '^')
                .for_each(|(i, _)| {
                    if beams.contains(&i) {
                        splits += 1;
                    }
                    beams.remove(&i);
                    beams.insert(i + 1);
                    if i > 0 {
                        beams.insert(i - 1);
                    }
                });
        } else if let Some(pos) = line.chars().position(|c| c == 'S') {
            beams.insert(pos);
        }
    });
    splits
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .fold(HashMap::<usize, usize>::new(), |mut beams, line| {
            if !beams.is_empty() {
                let mut new_beams: HashMap<usize, usize> = HashMap::new();
                for (&i, paths) in beams.iter() {
                    if line.chars().nth(i).unwrap() == '^' {
                        if i + 1 < line.len() {
                            *new_beams.entry(i + 1).or_insert(0) += paths;
                        }
                        if i > 0 {
                            *new_beams.entry(i - 1).or_insert(0) += paths;
                        }
                    } else {
                        *new_beams.entry(i).or_insert(0) += paths;
                    }
                }
                return new_beams;
            } else if let Some(pos) = line.chars().position(|c| c == 'S') {
                beams.insert(pos, 1);
            }
            beams
        })
        .into_values()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2025/day7.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1615);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 43560947406326);
    }
}
