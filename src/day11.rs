use crate::day09;
use ndarray::{s, Array2, ArrayViewMut2};
use std::collections::VecDeque;

fn increment_energies(grid: &mut ArrayViewMut2<Option<usize>>) -> VecDeque<(usize, usize)> {
    let mut queue = VecDeque::new();
    grid.indexed_iter_mut().for_each(|(index, x)| {
        if let Some(n) = x {
            *x = Some(*n + 1);
            if x.unwrap() == 10 {
                queue.push_back(index);
            }
        }
    });
    queue
}

fn step(grid: &mut ArrayViewMut2<Option<usize>>) -> usize {
    let mut queue = VecDeque::new();
    queue.append(&mut increment_energies(grid));
    let mut total_flashes = 0;

    while !queue.is_empty() {
        let flash_index = queue.pop_front().unwrap();
        let n = s![
            flash_index.0 - 1..flash_index.0 + 2,
            flash_index.1 - 1..flash_index.1 + 2
        ];
        let new_flash_indices = increment_energies(&mut grid.slice_mut(n));
        for index in new_flash_indices {
            let new_index = (index.0 + flash_index.0 - 1, index.1 + flash_index.1 - 1);
            if new_index != flash_index {
                queue.push_back(new_index);
            }
        }
    }

    grid.iter_mut().for_each(|x| {
        if let Some(n) = x {
            if *n > 9 {
                *x = Some(0);
                total_flashes += 1;
            }
        }
    });

    total_flashes
}

fn part1(grid: &Array2<Option<usize>>) -> usize {
    let mut g = grid.to_owned();
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += step(&mut g.view_mut());
    }
    flashes
}

fn part2(grid: &Array2<Option<usize>>) -> usize {
    let mut g = grid.to_owned();
    let mut i = 0;
    loop {
        i += 1;
        if step(&mut g.view_mut()) == 100 {
            return i;
        }
    }
}

pub fn run(lines: &[String]) {
    let input = day09::parse_input(lines);
    run_parts!(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> Vec<String> {
        vec![
            "5483143223".to_string(),
            "2745854711".to_string(),
            "5264556173".to_string(),
            "6141336146".to_string(),
            "6357385478".to_string(),
            "4167524645".to_string(),
            "2176841721".to_string(),
            "6882881134".to_string(),
            "4846848554".to_string(),
            "5283751526".to_string(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&day09::parse_input(&get_test_input())), 1656);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&day09::parse_input(&get_test_input())), 195);
    }
}
