advent_of_code::solution!(9);

pub fn _part_one(input: &str, preamble: usize) -> Option<u64> {
    let input = input
        .lines()
        .map(|row| row.parse().unwrap())
        .collect::<Vec<u64>>();

    'outer: for i in preamble..input.len() {
        let cypher_numbers = &input[i - preamble..i];
        let checked_number = input[i];

        for x in cypher_numbers.iter() {
            if cypher_numbers
                .iter()
                .any(|n| *x != *n && *x + *n == checked_number)
            {
                continue 'outer;
            }
        }

        return Some(checked_number);
    }

    None
}

pub fn part_one(input: &str) -> Option<u64> {
    _part_one(input, 25)
}

pub fn _part_two(input: &str, preamble: usize) -> Option<u64> {
    let part_one_result = _part_one(input, preamble).unwrap();

    let input = input
        .lines()
        .map(|row| row.parse().unwrap())
        .collect::<Vec<u64>>();

    for i in 0..input.len() {
        let mut acc = 0;
        let mut min = None;
        let mut max = None;
        for n in input.iter().skip(i) {
            acc += n;

            min = Some(min.map_or(n, |current_min: &u64| current_min.min(n)));
            max = Some(max.map_or(n, |current_max: &u64| current_max.max(n)));

            if acc == part_one_result {
                return Some(min.unwrap() + max.unwrap());
            }
        }
    }
    None
}
pub fn part_two(input: &str) -> Option<u64> {
    _part_two(input, 25)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = _part_one(&advent_of_code::template::read_file("examples", DAY), 5);
        assert_eq!(result, Some(127));
    }

    #[test]
    fn test_part_two() {
        let result = _part_two(&advent_of_code::template::read_file("examples", DAY), 5);
        assert_eq!(result, Some(62));
    }
}
