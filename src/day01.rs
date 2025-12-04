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

pub fn part2(input: &str) -> i64 {
    read_input(input)
        .iter()
        .scan(50, |val, &(letter, number)| {
            let x = if letter == 'R' { number } else { -number };
            let hit = if letter == 'R' {
                if *val != 0 && *val + (number % 100) >= 100 {
                    1
                } else {
                    0
                }
            } else {
                if *val != 0 && *val - (number % 100) <= 0 {
                    1
                } else {
                    0
                }
            };
            *val = modulo(*val + x, 100);
            Some(hit + number / 100)
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
