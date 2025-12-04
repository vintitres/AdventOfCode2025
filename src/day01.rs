fn read_input(input: &str) -> Vec<(char, i64)> {
    input
        .lines()
        .map(|l| {
            let (letter, number) = l.split_at(1);
            (
                letter.chars().next().unwrap(),
                number.parse::<i64>().unwrap(),
            )
        })
        .collect()
}

fn modulo(a: i64, b: i64) -> i64 {
    ((a % b) + b) % b
}

pub fn part1(input: &str) -> usize {
    read_input(input)
        .iter()
        .scan(50, |val, &(letter, number)| {
            *val = modulo(*val + if letter == 'R' { number } else { -number }, 100);
            Some(*val == 0)
        })
        .filter(|&x| x)
        .count()
}

pub fn part2(input: &str) -> u64 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2025/day1.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 962);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 1);
    }
}
