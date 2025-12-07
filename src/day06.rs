use itertools::Itertools;

pub fn part1(input: &str) -> u64 {
    let ops = input
        .lines()
        .skip_while(|line| line.trim_start().chars().next().unwrap().is_numeric())
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect_vec();
    input
        .lines()
        .take_while(|line| line.trim_start().chars().next().unwrap().is_numeric())
        .fold(
            ops.iter()
                .map(|&op| match op {
                    '*' => (1, op),
                    '+' => (0, op),
                    _ => panic!("Invalid operator {}", op),
                })
                .collect_vec(),
            |mut acc, line| {
                acc.iter_mut()
                    .zip(
                        line.split_ascii_whitespace()
                            .map(|s| s.parse::<u64>().unwrap())
                            .collect_vec(),
                    )
                    .for_each(|((a, op), b)| match op {
                        '*' => *a *= b,
                        '+' => *a += b,
                        _ => panic!("Invalid operator"),
                    });
                acc
            },
        )
        .iter()
        .map(|(a, _)| *a)
        .sum::<u64>()
}

pub fn part2(input: &str) -> usize {
    let max_len = input.lines().map(|line| line.len()).max().unwrap();
    let input = input
        .lines()
        .map(|line| line.chars().pad_using(max_len, |_| ' ').collect_vec())
        .collect_vec();
    let mut flipped_input = vec![vec!['-'; input.len()]; input[0].len()];
    println!("{:?}", flipped_input);
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2025/day6.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 4719804927602);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 5782);
    }
}
