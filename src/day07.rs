use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> usize {
    let mut beams = HashSet::new();
    let mut splits = 0;
    input.lines().for_each(|line| {
        if !beams.is_empty() {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '^')
                .for_each(|(i, _)| {
                    if beams.contains(&i) {
                        splits += 1;
                    }
                    beams.remove(&i);
                    beams.insert(i + 1);
                    if i > 0 {
                        beams.insert(i - 1);
                    }
                });
        } else if let Some(pos) = line.chars().position(|c| c == 'S') {
            beams.insert(pos);
        }
    });
    splits
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .enumerate()
        .fold(
            HashMap::<usize, HashSet<Vec<usize>>>::new(),
            |mut beams, (line_num, line)| {
                if !beams.is_empty() {
                    let mut new_beams: HashMap<usize, HashSet<Vec<usize>>> = HashMap::new();
                    for (&i, paths) in beams.iter() {
                        let new_paths: HashSet<Vec<usize>> = paths
                            .into_iter()
                            .map(|path| {
                                let mut new_path = path.clone();
                                new_path.push(i);
                                new_path
                            })
                            .collect();
                        if line.chars().nth(i).unwrap() == '^' {
                            if i + 1 < line.len() {
                                new_beams
                                    .entry(i + 1)
                                    .or_insert_with(HashSet::new)
                                    .extend(new_paths.clone());
                            }
                            if i > 0 {
                                new_beams
                                    .entry(i - 1)
                                    .or_insert_with(HashSet::new)
                                    .extend(new_paths);
                            }
                        } else {
                            new_beams
                                .entry(i)
                                .or_insert_with(HashSet::new)
                                .extend(new_paths);
                        }
                    }
                    println!("{:?}", new_beams);
                    println!("{}", line_num);
                    return new_beams;
                } else if let Some(pos) = line.chars().position(|c| c == 'S') {
                    let mut paths = HashSet::new();
                    paths.insert(vec![]);
                    beams.insert(pos, paths);
                }
                beams
            },
        )
        .into_values()
        .map(|paths| paths.len())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2025/day7.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1615);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 5782);
    }
}
