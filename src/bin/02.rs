advent_of_code::solution!(2);

pub struct Row<'a> {
    min: u64,
    max: u64,
    letter: char,
    password: &'a str,
}

pub fn parse_row(input: &str) -> Row {
    let mut parts = input.split(' ');

    let min_max = parts.next().unwrap();

    let mut min_max_parts = min_max.split('-');
    let min = min_max_parts.next().unwrap().parse().unwrap();
    let max = min_max_parts.next().unwrap().parse().unwrap();

    let letter = parts.next().unwrap().chars().next().unwrap();
    let password = parts.next().unwrap();

    Row {
        min,
        max,
        letter,
        password,
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let input: Vec<_> = input
        .lines()
        .map(|row| parse_row(row))
        .collect();

    let count = input
        .iter()
        .filter(|row| {
            let occurences = row.password.chars().filter(|c| *c == row.letter).count();

            occurences >= row.min as usize && occurences <= row.max as usize
        })
        .count();

    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input: Vec<_> = input
        .lines()
        .map(|row| parse_row(row))
        .collect();

    let count = input
        .iter()
        .filter(|row| {
            let cond1= row.password.chars().nth((row.min - 1) as usize).unwrap() == row.letter;
            let cond2= row.password.chars().nth((row.max - 1) as usize).unwrap() == row.letter;

            cond1 != cond2
        })
        .count();

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xtest_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
