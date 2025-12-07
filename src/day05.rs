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

pub fn part2(input: &str) -> usize {
    input.lines().count()
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
