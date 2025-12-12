use itertools::Itertools;

const PRESENTS_SIZES: [usize; 6] = [7, 5, 7, 6, 7, 7];

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .skip(30)
        .map(|line| {
            let (dimentions, presents) = line.split_once(": ").unwrap();
            let (x, y): (usize, usize) = dimentions
                .split("x")
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            let presents = presents
                .split(" ")
                .map(|s| s.parse::<usize>().unwrap())
                .collect_vec();
            let space = x * y;
            let presents_size = presents
                .iter()
                .enumerate()
                .map(|(i, p)| p * PRESENTS_SIZES[i])
                .sum::<usize>();
            if space < presents_size {
                0
            } else {
                1
            }
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
        include_str!("../input/2025/day12.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 583);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 5782);
    }
}
