use std::str::FromStr;
use itertools::Itertools;
advent_of_code::solution!(5);

#[derive(Debug)]
struct Seat {
    row: usize,
    column: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseSeatError;

impl FromStr for Seat {
    type Err = ParseSeatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (row, column) = s.split_at(7);

        let last_char = row.chars().last().ok_or(ParseSeatError)?;

        let row =
            row
                .chars()
                .fold((0, 127), |(min, max), c| {
                    let gap_len = max - min;
                    match c {
                        'F' => (min, max - gap_len / 2 - 1),
                        'B' => (min + gap_len / 2 + 1, max),
                        _ => panic!("Unexpected char {c}")
                    }
                });

        let row = match last_char {
            'F' => row.0,
            'B' => row.1,
            _ => panic!("Unexpected char {last_char}")
        };

        let last_char = column.chars().last().ok_or(ParseSeatError)?;

        let column =
            column
                .chars()
                .fold((0, 7), |(min, max), c| {
                    let gap_len = max - min;
                    match c {
                        'L' => (min, max - gap_len / 2 - 1),
                        'R' => (min + gap_len / 2 + 1, max),
                        _ => panic!("Unexpected char {c}")
                    }
                });

        let column = match last_char {
            'R' => column.0,
            'L' => column.1,
            _ => panic!("Unexpected char {last_char}")
        };


        Ok(Seat {
            row,
            column,
        })
    }
}

impl Seat {
    fn id(&self) -> usize {
        self.row * 8 + self.column
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    input
        .lines()
        .map(|row| row.parse().unwrap())
        .map(|seat: Seat| seat.id())
        .max()
}

pub fn part_two(input: &str) -> Option<usize> {
    input
        .lines()
        .map(|row| row.parse().unwrap())
        .map(|seat: Seat| seat.id())
        .sorted()
        .tuple_windows::<(_,_)>()
        .find(|(prev, next)| *next != *prev + 1)
        .map(|(left_seat, _)| left_seat + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(820));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(120));
    }
}
