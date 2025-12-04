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

pub fn part1(input: &str) -> usize {
    read_input(input)
        .iter()
        .scan(50, |val, &(letter, number)| {
            let x = if letter == 'R' {
                number % 100
            } else {
                100 - number % 100
            };
            *val = (*val + x) % 100;
            Some(*val == 0)
        })
        .filter(|&x| x)
        .count()
}

pub fn part2(input: &str) -> i64 {
    read_input(input)
        .iter()
        .scan(50, |val, &(letter, number)| {
            let x = if letter == 'R' {
                number % 100
            } else {
                100 - number % 100
            };
            let mut hits = number / 100;
            if *val != 0
                && ((letter == 'R' && *val + x >= 100) || (letter != 'R' && *val <= number % 100))
            {
                hits += 1;
            }
            *val = (*val + x) % 100;
            Some(hits)
        })
        .sum()
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
        assert_eq!(part2(input()), 5782);
    }
}
