use std::iter;

use fxhash::FxHashMap;
use itertools::Itertools;

advent_of_code::solution!(10);


pub fn part_one(input: &str) -> Option<u32> {
    let (jolt_1, jolt_3) = input
        .lines()
        .map(|row| row.parse::<u32>().unwrap())
        .chain(iter::once(0))
        .sorted()
        .tuple_windows()
        .fold((0, 0), |(jolt_1, jolt_3), (prev, next)| match next - prev {
            3 => (jolt_1, jolt_3 + 1),
            1 => (jolt_1 + 1, jolt_3),
            _ => (jolt_1, jolt_3)
        });
    Some(jolt_1 * (jolt_3 + 1))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum_map = FxHashMap::default();
    sum_map.insert(0, 1);

    let sum_map = input
        .lines()
        .map(|row| row.parse::<usize>().unwrap())
        .sorted()
        .fold(sum_map, |mut sum_map, i| {
            let sum = (1..=3)
                .filter_map(|j| i.checked_sub(j).and_then(|key| sum_map.get(&key)))
                .sum();

            sum_map.insert(i, sum);

            sum_map
        });

    let max_key = sum_map.keys().max().unwrap();

    Some(*sum_map.get(max_key).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(220));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(19208));
    }
}
