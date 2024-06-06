use std::collections::HashSet;
use itertools::Itertools;
advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    Some(input
        .split("\n\n")
        .map(|group|
            group
                .chars()
                .filter(|c| *c != '\n')
                .unique()
                .count()
        )
        .sum()
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|person| person.chars().collect::<HashSet<_>>())
                .reduce(|a, b| a.intersection(&b).copied().collect::<HashSet<_>>())
                .unwrap()
                .len()
        })
        .sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
