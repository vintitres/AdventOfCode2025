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

pub fn part2(input: &str) -> usize {
    input.lines().count()
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

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 5782);
    }
}
