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
    input.lines().count()
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

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 5782);
    }
}
