fn is_invalid(num: &u64) -> bool {
    is_invalid_x(&num.to_string(), 2)
}

fn is_invalid_x(num_str: &str, x: usize) -> bool {
    let num_len = num_str.len();
    if num_len % x != 0 {
        return false;
    }
    let section_len = num_len / x;
    let num_str_0 = &num_str[0..section_len];
    (1..x).all(|i| num_str_0 == &num_str[i * section_len..(i + 1) * section_len])
}

fn is_invalid_any(num: &u64) -> bool {
    let num_str = num.to_string();
    let num_len = num_str.len();
    (2..=num_len).any(|x| is_invalid_x(&num_str, x))
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

pub fn part2(input: &str) -> u64 {
    input
        .trim()
        .split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();
            (start..=end).filter(is_invalid_any).sum::<u64>()
        })
        .sum()
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 20077272987);
    }

    #[test]
    fn test_is_invalid_any() {
        assert!(is_invalid_any(&22));
        assert!(is_invalid_any(&11));
        assert!(is_invalid_any(&111));
        assert!(is_invalid_any(&111));
    }
}
