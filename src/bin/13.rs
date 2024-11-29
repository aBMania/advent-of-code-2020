use advent_of_code::crt::solve_crt;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    let [start, buses] = input.lines().collect::<Vec<_>>()[..] else {
        panic!("Invalid input")
    };
    let start = start.parse::<u32>().ok()?;

    let best_bus = buses
        .split(',')
        .filter_map(|bus| bus.parse::<u32>().ok())
        .map(|bus| (bus, bus - (start % bus)))
        .min_by(|(_bus, next), (_bus2, next2)| next.cmp(next2));

    Some(best_bus?.0 * best_bus?.1)
}

pub fn part_two(input: &str) -> Option<i64> {
    let [_, buses] = input.lines().collect::<Vec<_>>()[..] else {
        panic!("Invalid input")
    };

    let (a_i, n_i): (Vec<_>, Vec<_>) = buses
        .split(',')
        .enumerate()
        .filter_map(|(i, bus)| bus.parse().ok().map(|bus: i64| (-(i as i64), bus)))
        .unzip();

    let (solution, _) = solve_crt(a_i.as_ref(), n_i.as_ref()).unwrap();

    Some(solution)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(295));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1068781));
    }
}
