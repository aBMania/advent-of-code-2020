use itertools::iproduct;
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines();

    let numbers: Vec<_> = lines
        .map(|line| line.parse::<u64>().ok().unwrap())
        .collect();

    let (i, j) = iproduct!(numbers.clone(), numbers)
        .find(|(i, j) | i+j == 2020)
        .unwrap();

    Some(i*j)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines();

    let numbers: Vec<_> = lines
        .map(|line| line.parse::<u64>().ok().unwrap())
        .collect();

    let (i, j, k) = iproduct!(numbers.clone(), numbers.clone(), numbers)
        .find(|(i, j, k) | i+j+k == 2020)
        .unwrap();

    Some(i*j*k)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(514579));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(241861950));
    }
}
