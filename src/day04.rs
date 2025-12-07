use std::collections::HashSet;

use itertools::Itertools;

type Pos = (isize, isize);

struct World {
    map: Vec<Vec<char>>,
}

impl World {
    fn read(input: &str) -> World {
        let map = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        World { map }
    }
    fn get(&self, pos: Pos) -> Option<char> {
        if let Ok(i) = usize::try_from(pos.0) {
            if let Ok(j) = usize::try_from(pos.1) {
                if let Some(map_row) = self.map.get(i) {
                    return map_row.get(j).copied();
                }
            }
        }
        None
    }

    fn width(&self) -> usize {
        self.map.len()
    }

    fn height(&self) -> usize {
        self.map[0].len()
    }

    fn check_rolls(&self, pos: Pos, removed: &HashSet<Pos>) -> bool {
        !removed.contains(&pos)
            && self.get(pos) == Some('@')
            && vec![
                (pos.0, pos.1 + 1),
                (pos.0, pos.1 - 1),
                (pos.0 + 1, pos.1),
                (pos.0 - 1, pos.1),
                (pos.0 - 1, pos.1 - 1),
                (pos.0 + 1, pos.1 + 1),
                (pos.0 + 1, pos.1 - 1),
                (pos.0 - 1, pos.1 + 1),
            ]
            .iter()
            .filter(|p| !removed.contains(p) && self.get(**p) == Some('@'))
            .count()
                < 4
    }
}

pub fn part1(input: &str) -> usize {
    let w = World::read(input);
    (0..w.height())
        .flat_map(|i| (0..w.width()).map(move |j| (i, j)))
        .filter(|(i, j)| w.check_rolls((*i as isize, *j as isize), &HashSet::new()))
        .count()
}

pub fn part2(input: &str) -> usize {
    let w = World::read(input);
    let mut removed = HashSet::new();
    loop {
        let new_removed = (0..w.height())
            .flat_map(|i| (0..w.width()).map(move |j| (i as isize, j as isize)))
            .filter(|(i, j)| w.check_rolls((*i as isize, *j as isize), &removed))
            .map(|(i, j)| (i as isize, j as isize))
            .collect_vec();
        if new_removed.is_empty() {
            break;
        }
        removed.extend(new_removed);
    }
    removed.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2025/day4.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1533);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 9206);
    }
}
