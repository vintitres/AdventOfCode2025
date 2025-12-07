use std::collections::HashSet;

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
        .fold(
            vec![0; input.lines().next().unwrap().len()],
            |beams, line| {
                let mut new_beams = beams.clone();
                if let Some(pos) = line.chars().position(|c| c == 'S') {
                    new_beams[pos] = 1;
                    return new_beams;
                }
                beams
                    .iter()
                    .enumerate()
                    .filter(|(i, &beam)| beam > 0 && line.chars().nth(*i).unwrap() == '^')
                    .for_each(|(i, &beam)| {
                        if i + 1 < line.len() {
                            new_beams[i + 1] += beam;
                        }
                        if i > 0 {
                            new_beams[i - 1] += beam;
                        }
                        new_beams[i] = 0;
                    });
                new_beams
            },
        )
        .iter()
        .sum::<usize>()
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
