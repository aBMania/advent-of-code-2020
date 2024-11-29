use advent_of_code::grid::{input_to_grid, CustomGrid};
advent_of_code::solution!(11);

fn apply_iteration(grid: &mut CustomGrid<char>) -> bool {
    let mut tmp_grid: CustomGrid<char> = grid.clone();
    let mut changed = false;

    for ((x, y), c) in grid.indexed_iter() {
        match (
            c,
            grid.iter_diagonal_neighbors(x, y)
                .filter(|(_, &c)| c == '#')
                .count(),
        ) {
            ('L', 0) => {
                *tmp_grid.get_mut(x, y).unwrap() = '#';
                changed = true;
            }
            ('#', n) if n >= 4 => {
                *tmp_grid.get_mut(x, y).unwrap() = 'L';
                changed = true;
            }
            (_, _) => {}
        }
    }

    *grid = tmp_grid;

    changed
}

fn apply_iteration_2(grid: &mut CustomGrid<char>) -> bool {
    let mut tmp_grid: CustomGrid<char> = grid.clone();
    let mut changed = false;

    for ((x, y), c) in grid.indexed_iter() {
        let extented_diagonal_neighbor_count = [
            (-1isize, -1isize),
            (-1isize, 0isize),
            (-1isize, 1isize),
            (0isize, -1isize),
            (0isize, 1isize),
            (1isize, -1isize),
            (1isize, 0isize),
            (1isize, 1isize),
        ]
        .iter()
        .filter_map(|(y_offset, x_offset)| {
            let (mut current_x, mut current_y) = (x, y);
            loop {
                if ((current_x as isize) < -*x_offset) || ((current_y as isize) < -*y_offset) {
                    break None;
                }
                current_x = ((current_x as isize) + x_offset) as usize;
                current_y = ((current_y as isize) + y_offset) as usize;
                match grid.get(current_x, current_y) {
                    None => break None,
                    Some(c) => match c {
                        '.' => continue,
                        c => break Some(c),
                    },
                }
            }
        })
        .filter(|&&c| c == '#')
        .count();

        match (c, extented_diagonal_neighbor_count) {
            ('L', 0) => {
                *tmp_grid.get_mut(x, y).unwrap() = '#';
                changed = true;
            }
            ('#', n) if n >= 5 => {
                *tmp_grid.get_mut(x, y).unwrap() = 'L';
                changed = true;
            }
            (_, _) => {}
        }
    }

    *grid = tmp_grid;

    changed
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: CustomGrid<char> = input_to_grid(input).unwrap();

    while apply_iteration(&mut grid) {
        // Wait for no changes to happen after an iteration
    }

    Some(grid.iter().filter(|&&c| c == '#').count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: CustomGrid<char> = input_to_grid(input).unwrap();

    while apply_iteration_2(&mut grid) {
        // Wait for no changes to happen after an iteration
    }

    Some(grid.iter().filter(|&&c| c == '#').count() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(26));
    }
}
