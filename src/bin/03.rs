advent_of_code::solution!(3);

fn with_slope(input: &str, slope_right: usize, slope_down: usize) -> usize {
    input
        .lines()
        .step_by(slope_down)
        .enumerate()
        .filter(|(i, line)| {
            println!("{i}, {} {line}, {}", (slope_right * *i) % line.len(), line.chars().nth((slope_right * *i) % line.len()).unwrap() == '#');
            line.chars().nth((slope_right * *i) % line.len()).unwrap() == '#'
        })
        .count()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(with_slope(input, 3, 1))
}

pub fn part_two(input: &str) -> Option<usize> {
    let result = 
        with_slope(input, 1, 1) *
        with_slope(input, 3, 1) *
        with_slope(input, 5, 1) *
        with_slope(input, 7, 1) *
        with_slope(input, 1, 2);
    
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(336));
    }
}
