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

fn find_parity_buttons(joltage: &[usize], buttons: &[Vec<usize>]) -> Vec<Vec<Vec<usize>>> {
    (1..=buttons.len())
        .map(|i| {
            buttons
                .iter()
                .combinations(i)
                .map(|buttons_set| {
                    if joltage.iter().enumerate().all(|(i, &j)| {
                        let c = buttons_set
                            .iter()
                            .filter(|button| button.contains(&i))
                            .count();
                        c <= j && (j - c) % 2 == 0
                    }) {
                        Some(
                            buttons_set
                                .iter()
                                .map(|button| button.iter().map(|&bit| bit).collect_vec())
                                .collect_vec(),
                        )
                    } else {
                        None
                    }
                })
                .flatten()
        })
        .flatten()
        .collect_vec()
}

fn min_push_joltage(
    joltage: &[usize],
    buttons: &[Vec<usize>],
    mem: &mut HashMap<Vec<usize>, Option<u64>>,
) -> Option<u64> {
    if joltage.iter().all(|&j| j == 0) {
        return Some(0);
    }
    if let Some(&result) = mem.get(joltage) {
        return result;
    }
    if joltage.iter().all(|&j| j % 2 == 0) {
        let new_joltage = joltage.iter().map(|&j| j / 2).collect_vec();
        return if let Some(res) = min_push_joltage(&new_joltage, &buttons, mem) {
            Some(2 * res)
        } else {
            None
        };
    }
    let result = find_parity_buttons(joltage, buttons)
        .iter()
        .map(|parity_buttons| {
            // println!(
            //     "joltage: {:?}, parity_buttons: {:?}",
            //     joltage, parity_buttons
            // );
            let new_joltage = joltage
                .iter()
                .enumerate()
                .map(|(i, &j)| (j - parity_buttons.iter().filter(|b| b.contains(&i)).count()) / 2)
                .collect_vec();
            if let Some(sub_presses) = min_push_joltage(&new_joltage, &buttons, mem) {
                Some(parity_buttons.len() as u64 + 2 * sub_presses)
            } else {
                None
            }
        })
        .flatten()
        .min();
    // println!("joltage: {:?}, result: {:?}", joltage, result);
    mem.insert(joltage.to_vec(), result);
    result
}

pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
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
            let mpl = min_push_joltage(&joltage, &buttons.to_vec(), &mut HashMap::new());
            println!("{}: {:?}", i + 1, mpl);
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
        assert_eq!(part2(input()), 1);
    }
}
