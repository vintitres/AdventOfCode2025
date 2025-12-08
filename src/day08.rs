use itertools::Itertools;

fn distance3d(a: &(i32, i32, i32), b: &(i32, i32, i32)) -> i64 {
    ((a.0 - b.0) as i64).abs().pow(2)
        + ((a.1 - b.1) as i64).abs().pow(2)
        + ((a.2 - b.2) as i64).abs().pow(2)
}

pub fn part1(input: &str) -> usize {
    let boxes = input
        .lines()
        .map(|line| {
            let (x, y, z) = line
                .split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap();
            (x, y, z)
        })
        .collect_vec();

    let dist_pairs = (0..boxes.len())
        .combinations(2)
        .map(|pair| {
            let (&i, &j) = pair.iter().collect_tuple().unwrap();
            (distance3d(&boxes[i], &boxes[j]), (i, j))
        })
        .sorted()
        .collect_vec();

    let mut circuts_sets = (0..boxes.len()).map(|i| vec![i]).collect_vec();
    let mut circuts_index = (0..boxes.len()).collect_vec();

    for (_, (mut i, mut j)) in &dist_pairs[0..1000] {
        if circuts_sets[circuts_index[i]].len() > circuts_sets[circuts_index[j]].len() {
            std::mem::swap(&mut i, &mut j);
        }
        let to_move = circuts_sets[circuts_index[i]].clone();
        circuts_sets[circuts_index[i]].clear();
        for k in to_move {
            circuts_sets[circuts_index[j]].push(k);
            circuts_index[k] = circuts_index[j];
        }
    }
    circuts_sets
        .iter()
        .map(|set| set.len())
        .sorted()
        .rev()
        .take(3)
        .product()
}

pub fn part2(input: &str) -> i64 {
    let boxes = input
        .lines()
        .map(|line| {
            let (x, y, z) = line
                .split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap();
            (x, y, z)
        })
        .collect_vec();

    let dist_pairs = (0..boxes.len())
        .combinations(2)
        .map(|pair| {
            let (&i, &j) = pair.iter().collect_tuple().unwrap();
            (distance3d(&boxes[i], &boxes[j]), (i, j))
        })
        .sorted()
        .collect_vec();

    let mut circuts_sets = (0..boxes.len()).map(|i| vec![i]).collect_vec();
    let mut circuts_index = (0..boxes.len()).collect_vec();

    for (_, (mut i, mut j)) in &dist_pairs {
        if circuts_sets[circuts_index[i]].len() > circuts_sets[circuts_index[j]].len() {
            std::mem::swap(&mut i, &mut j);
        }
        let to_move = circuts_sets[circuts_index[i]].clone();
        circuts_sets[circuts_index[i]].clear();
        for k in to_move {
            circuts_sets[circuts_index[j]].push(k);
            circuts_index[k] = circuts_index[j];
        }
        if circuts_sets[circuts_index[j]].len() == boxes.len() {
            return boxes[i].0 as i64 * boxes[j].0 as i64;
        }
    }
    panic!("not connected")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2025/day8.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 46398);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 8141888143);
    }
}
