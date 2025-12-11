use std::collections::{HashSet, VecDeque};

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

fn min_push_joltage(
    needed_joltage: &[usize],
    buttons: &Vec<&Vec<usize>>,
    buttons_start: usize,
) -> Option<u64> {
    if needed_joltage.iter().all(|&j| j == 0) {
        return Some(0);
    }
    buttons
        .iter()
        .skip(buttons_start)
        .map(|button| {
            if button.iter().any(|&i| needed_joltage[i] == 0) {
                return None;
            }
            let new_needed_joltage = needed_joltage
                .iter()
                .enumerate()
                .map(|(i, &j)| j - if button.contains(&i) { 1 } else { 0 })
                .collect_vec();
            let mpj1 = match min_push_joltage(&new_needed_joltage, buttons, buttons_start) {
                Some(j) => Some(j + 1),
                None => None,
            };
            let mpj2 = match min_push_joltage(&new_needed_joltage, buttons, buttons_start + 1) {
                Some(j) => Some(j + 1),
                None => None,
            };
            vec![mpj1, mpj2].into_iter().flatten().min()
        })
        .flatten()
        .min()
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
            let joltage = &joltage[0];
            let mpl = min_push_joltage(
                &joltage,
                &buttons
                    .iter()
                    .sorted_by_key(|button| {
                        (
                            button.iter().map(|&i| joltage[i]).min().unwrap(),
                            10 - button.len(),
                        )
                    })
                    .collect_vec(),
                0,
            );
            println!("{:?}", mpl);
            mpl
        })
        .flatten()
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
