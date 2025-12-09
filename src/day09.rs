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

pub fn part2(input: &str) -> usize {
    input.lines().count()
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

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 5782);
    }
}
