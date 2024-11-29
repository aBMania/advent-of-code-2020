advent_of_code::solution!(7);

use fxhash::{FxBuildHasher, FxHashMap, FxHashSet};
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq)]
struct ParsingError {
    reason: String,
}

impl ParsingError {
    fn new(reason: &str) -> Self {
        ParsingError {
            reason: reason.to_owned(),
        }
    }
}

fn parse_row(row: &str) -> Result<(&str, HashMap<&str, u32, FxBuildHasher>), ParsingError> {
    let (bag, content) = row
        .split_once(" bags contain")
        .ok_or(ParsingError::new("No bags"))?;

    let mut map = FxHashMap::default();

    if content.contains("no other bags.") {
        return Ok((bag, map));
    }

    for subbag in content[..content.len() - 1].split(',') {
        let n = subbag
            .chars()
            .nth(1)
            .ok_or(ParsingError::new("Bad str len at n bag parsing"))?
            .to_digit(10)
            .ok_or(ParsingError::new(&format!("Invalid number {}", subbag)))?;

        let lookback = match n > 1 {
            true => 5,
            false => 4,
        };

        let a = &subbag[3..subbag.len() - lookback];

        map.insert(a, n);
    }

    Ok((bag, map))
}

pub fn part_one(input: &str) -> Option<u32> {
    let edges = match input
        .lines()
        .map(|row| parse_row(row))
        .collect::<Result<Vec<_>, ParsingError>>()
    {
        Ok(x) => x,
        Err(_) => return None,
    };

    let mut reverse_edges: HashMap<&str, Vec<&str>, FxBuildHasher> = FxHashMap::default();

    for (bag, content) in edges {
        for (content_bag, _) in content {
            reverse_edges
                .entry(content_bag)
                .and_modify(|wrapping_bags| wrapping_bags.push(bag))
                .or_insert(vec![bag]);
        }
    }

    let mut visited: HashSet<&str, _> = FxHashSet::default();
    let mut to_visit = vec!["shiny gold"];

    while let Some(visiting) = to_visit.pop() {
        if visited.contains(visiting) {
            continue;
        }

        visited.insert(visiting);

        if let Some(visiting_reverse_edges) = reverse_edges.get(visiting) {
            for edge in visiting_reverse_edges {
                to_visit.push(edge);
            }
        }
    }

    Some(visited.len() as u32 - 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let edges = match input
        .lines()
        .map(|row| parse_row(row))
        .collect::<Result<FxHashMap<_, _>, ParsingError>>()
    {
        Ok(x) => x,
        Err(_) => return None,
    };

    let mut to_visit = vec![("shiny gold", 1)];
    let mut total = 0;

    while let Some((visiting, n)) = to_visit.pop() {
        if let Some(visiting_edges) = edges.get(visiting) {
            for (edge, m) in visiting_edges {
                total += m * n;
                to_visit.push((edge, m * n));
            }
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(126));
    }
}
