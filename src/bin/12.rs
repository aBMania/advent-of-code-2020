use advent_of_code::Direction;
use std::str::FromStr;

advent_of_code::solution!(12);

#[derive(Debug)]
enum Action {
    Direction(Direction),
    Left,
    Right,
    Forward,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseActionError;

impl FromStr for Action {
    type Err = ParseActionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(Action::Direction(Direction::North)),
            "E" => Ok(Action::Direction(Direction::East)),
            "S" => Ok(Action::Direction(Direction::South)),
            "W" => Ok(Action::Direction(Direction::West)),
            "L" => Ok(Action::Left),
            "R" => Ok(Action::Right),
            "F" => Ok(Action::Forward),
            _ => Err(ParseActionError),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (east, north, _) = input
        .lines()
        .map(|row| {
            let (action, n) = row.split_at(1);
            (action.parse().unwrap(), n.parse::<i32>().unwrap())
        })
        .fold(
            (0, 0, Direction::East),
            |(east, north, facing), (action, n)| match (action, n, facing) {
                (Action::Direction(Direction::North), n, facing) => (east + n, north, facing),
                (Action::Direction(Direction::South), n, facing) => (east - n, north, facing),
                (Action::Direction(Direction::East), n, facing) => (east, north + n, facing),
                (Action::Direction(Direction::West), n, facing) => (east, north - n, facing),
                (_, 180, facing) => (east, north, facing.opposite()),
                (Action::Left, 90, facing) | (Action::Right, 270, facing) => {
                    (east, north, facing.rotate_left())
                }
                (Action::Left, 270, facing) | (Action::Right, 90, facing) => {
                    (east, north, facing.rotate_right())
                }
                (Action::Forward, n, Direction::North) => (east + n, north, Direction::North),
                (Action::Forward, n, Direction::South) => (east - n, north, Direction::South),
                (Action::Forward, n, Direction::East) => (east, north + n, Direction::East),
                (Action::Forward, n, Direction::West) => (east, north - n, Direction::West),
                (_, _, _) => panic!("Unknown case"),
            },
        );

    Some(east.abs() as u32 + north.abs() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (east, north, _, _) = input
        .lines()
        .map(|row| {
            let (action, n) = row.split_at(1);
            (action.parse().unwrap(), n.parse::<i32>().unwrap())
        })
        .fold(
            (0, 0, 10, 1),
            |(east, north, wp_east, wp_north), (action, n)| match (action, n) {
                (Action::Direction(Direction::North), n) => (east, north, wp_east, wp_north + n),
                (Action::Direction(Direction::East), n) => (east, north, wp_east + n, wp_north),
                (Action::Direction(Direction::South), n) => (east, north, wp_east, wp_north - n),
                (Action::Direction(Direction::West), n) => (east, north, wp_east - n, wp_north),
                (Action::Left | Action::Right, 180) => (east, north, -wp_east, -wp_north),
                (Action::Left, 90) | (Action::Right, 270) => (east, north, -wp_north, wp_east),
                (Action::Left, 270) | (Action::Right, 90) => (east, north, wp_north, -wp_east),
                (Action::Forward, n) => {
                    (east + n * wp_east, north + n * wp_north, wp_east, wp_north)
                }
                (_, _) => panic!("Unknown case"),
            },
        );

    Some(east.abs() as u32 + north.abs() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(286));
    }
}
