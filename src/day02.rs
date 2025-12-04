fn is_invalid(num: &u64) -> bool {
    let num_str = num.to_string();
    if num_str.len() % 2 == 0 {
        num_str
            .chars()
            .zip(num_str.chars().skip(num_str.len() / 2))
            .all(|(a, b)| a == b)
    } else {
        false
    }
}

pub fn part1(input: &str) -> u64 {
    input
        .trim()
        .split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();
            (start..=end).filter(is_invalid).sum::<u64>()
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    input.lines().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2025/day2.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 18700015741);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 5782);
    }
}
