use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

fn min_push(lights: &u64, buttons: &[u64]) -> usize {
    let mut queue = VecDeque::from([(*lights, 0)]);
    let mut visited = HashSet::from([*lights]);
    while let Some((lights, pushes)) = queue.pop_front() {
        for button in buttons {
            let new_lights = lights ^ button;
            if new_lights == 0 {
                return pushes + 1;
            }
            if visited.insert(new_lights) {
                queue.push_back((new_lights, pushes + 1));
            }
        }
    }
    unreachable!()
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (lights, buttons) = line.split_once(' ').unwrap();
            let buttons = buttons
                .split(' ')
                .filter(|&button| button.starts_with('('))
                .map(|button| button.strip_suffix(')').unwrap().strip_prefix('(').unwrap())
                .map(|button| {
                    button
                        .split(',')
                        .map(|bit_index| 2_u64.pow(bit_index.parse::<u32>().unwrap()))
                        .sum::<u64>()
                })
                .collect_vec();
            let lights = lights
                .strip_prefix('[')
                .unwrap()
                .strip_suffix(']')
                .unwrap()
                .chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(|(i, _)| 2_u64.pow(i as u32))
                .sum::<u64>();
            min_push(&lights, &buttons)
        })
        .sum()
}

fn min_push_joltage(_joltage: &[usize], _buttons: &[Vec<usize>]) -> u64 {
    1
    /*
    * [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}

    *
    *    3 5 4 7
    *    -------
    *    0 0 0 1
    *    0 1 0 1
    *    0 0 1 0
    *    0 0 1 1
    *    1 0 1 0
    *    1 1 0 0
    *
    */
}

pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let buttons = line
                .split(' ')
                .skip(1)
                .map(|button| &button[1..button.len() - 1])
                .map(|button| {
                    button
                        .split(',')
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect_vec()
                })
                .collect_vec();

            let (buttons, joltage) = buttons.split_at(buttons.len() - 1);
            min_push_joltage(&joltage[0], &buttons)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2025/day10.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 507);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 5782);
    }
}
