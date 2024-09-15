use advent_of_code::Direction;

advent_of_code::solution!(12);

#[derive(Debug)]
enum Action {
    Direction(Direction),
    Left,
    Right,
    Forward,
}

pub fn part_one(input: &str) -> Option<u32> {
    let (east, north, _) = input
        .lines()
        .map(|row| {
            let (action, n) = row.split_at(1);

            let action = match action {
                "N" => Action::Direction(Direction::North),
                "E" => Action::Direction(Direction::East),
                "S" => Action::Direction(Direction::South),
                "W" => Action::Direction(Direction::West),
                "L" => Action::Left,
                "R" => Action::Right,
                "F" => Action::Forward,
                _ => panic!("Unknown action")
            };

            (action, n.parse::<i32>().unwrap())
        }).fold((0, 0, Direction::East), |(east, north, facing), (action, n)|
        match (action, n, facing) {
            (Action::Direction(Direction::North), n, facing) => (east + n, north, facing),
            (Action::Direction(Direction::South), n, facing) => (east - n, north, facing),
            (Action::Direction(Direction::East), n, facing) => (east, north + n, facing),
            (Action::Direction(Direction::West), n, facing) => (east, north - n, facing),
            (_, 180, facing) => (east, north, facing.opposite()),
            (Action::Left, 90, facing) => (east, north, facing.rotate_left()),
            (Action::Left, 270, facing) => (east, north, facing.rotate_right()),
            (Action::Right, 90, facing) => (east, north, facing.rotate_right()),
            (Action::Right, 270, facing) => (east, north, facing.rotate_left()),
            (Action::Forward, n, Direction::North) => (east + n, north, Direction::North),
            (Action::Forward, n, Direction::South) => (east - n, north, Direction::South),
            (Action::Forward, n, Direction::East) => (east, north + n, Direction::East),
            (Action::Forward, n, Direction::West) => (east, north - n, Direction::West),
            (_, _, _) => panic!("Unknown case")
        },
    );

    Some(east.abs() as u32 + north.abs() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
