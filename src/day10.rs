use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

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
    buttons: &Vec<Vec<usize>>,
    presses: u64,
    mut min_presses: &mut u64,
) -> Option<u64> {
    if presses >= *min_presses {
        return None;
    }
    if needed_joltage.iter().all(|&j| j == 0) {
        *min_presses = presses.min(*min_presses);
        return Some(0);
    }
    // let (i, &j) = needed_joltage
    //     .iter()
    //     .enumerate()
    //     .filter(|&(_, &j)| j > 0)
    //     .min_by_key(|&(i, j)| (buttons.iter().filter(|b| b.contains(&i)).count(), j))
    //     .unwrap();
    if let Some((bi, b)) = buttons.iter().enumerate().min_by_key(|(_, b)| b.len()) {
        let mut new_buttons = buttons.clone();
        new_buttons.remove(bi);
        let max_p = b.iter().map(|bb| needed_joltage[*bb]).min().unwrap();
        (0..=max_p)
            .rev()
            .map(|bp| {
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
                match min_push_joltage(
                    &new_needed_joltage,
                    &new_buttons,
                    presses + bp as u64,
                    &mut min_presses,
                ) {
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

fn min_push_joltage2(expected_joltage: &[usize], buttons: &[Vec<usize>]) -> Option<u64> {
    let buttons = &buttons
        .iter()
        .sorted_by_key(|b| b.iter().map(|i| expected_joltage[*i]).min())
        .map(|b| b.clone())
        .collect_vec();
    let mut seen = HashMap::new();
    let mut queue = BTreeSet::new();
    queue.insert((0, vec![0; expected_joltage.len()]));
    while let Some((presses, joltage)) = queue.pop_first() {
        // println!("Presses: {}, Joltage: {:?}", presses, joltage);
        let mut eq = true;
        let mut gt = false;
        for (j, e) in joltage.iter().zip(expected_joltage.iter()) {
            if j > e {
                gt = true;
                break;
            }
            if j != e {
                eq = false;
            }
        }
        if gt {
            continue;
        }
        if eq {
            return Some(presses);
        }
        if let Some(&best_presses) = seen.get(&joltage) {
            if presses >= best_presses {
                continue;
            }
        }
        seen.insert(joltage.clone(), presses);
        for button in buttons {
            let max_presses = button.iter().map(|&i| i).min().unwrap();
            for new_presses in (1..=max_presses).rev() {
                let new_joltage = joltage
                    .iter()
                    .enumerate()
                    .map(|(i, &j)| {
                        if button.contains(&i) {
                            j + new_presses
                        } else {
                            j
                        }
                    })
                    .collect_vec();
                queue.insert((presses + new_presses as u64, new_joltage));
            }
        }
    }
    None
}

pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .enumerate()
        .skip(6)
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
            let mut min_presses = u64::MAX;
            let mpl = min_push_joltage(&joltage, &buttons.to_vec(), 0, &mut min_presses);
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
