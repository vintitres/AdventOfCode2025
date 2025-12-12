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

fn min_push_joltage(needed_joltage: &[usize], buttons: &Vec<Vec<usize>>) -> Option<u64> {
    if needed_joltage.iter().all(|&j| j == 0) {
        return Some(0);
    }
    let (i, &j) = needed_joltage
        .iter()
        .enumerate()
        .filter(|&(_, &j)| j > 0)
        .min_by_key(|&(i, j)| (buttons.iter().filter(|b| b.contains(&i)).count(), j))
        .unwrap();
    if buttons
        .iter()
        .enumerate()
        .filter(|(_, b)| b.contains(&i))
        .count()
        > 2
    {
        panic!(">2");
    }
    if let Some((bi, b)) = buttons
        .iter()
        .enumerate()
        .filter(|(_, b)| b.contains(&i))
        .next()
    {
        let mut new_buttons = buttons.clone();
        new_buttons.remove(bi);
        [0, j]
            .iter()
            .map(|&bp| {
                // println!(
                //     "needed_joltage: {:?}, buttons: {:?}, i: {}, j: {}, bi: {}, b: {:?}, bp: {}",
                //     needed_joltage, buttons, i, j, bi, b, bp
                // );
                if needed_joltage
                    .iter()
                    .enumerate()
                    .any(|(i, &j)| b.contains(&i) && j < bp)
                {
                    return None;
                }
                let new_needed_joltage = needed_joltage
                    .iter()
                    .enumerate()
                    .map(|(i, &j)| j - if b.contains(&i) { bp } else { 0 })
                    .collect_vec();
                match min_push_joltage(&new_needed_joltage, &new_buttons) {
                    Some(min_joltage) => Some(bp as u64 + min_joltage),
                    None => None,
                }
            })
            .flatten()
            .min()
    } else {
        None
    }
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
            let mpl = min_push_joltage(&joltage, &buttons.to_vec());
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
        assert_eq!(part2(input()), 5782);
    }
}
