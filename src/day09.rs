use ndarray::{s, Array, Array2, ArrayView2};
use std::collections::VecDeque;

fn is_minimum(neighborhood: ArrayView2<Option<usize>>) -> bool {
    let center = neighborhood.get((1, 1)).unwrap().unwrap();
    let mut minimum = true;

    if let Some(Some(left_neighbor)) = neighborhood.get((1, 0)) {
        if left_neighbor < &center {
            minimum = false;
        }
    }

    if let Some(Some(up_neighbor)) = neighborhood.get((0, 1)) {
        if up_neighbor < &center {
            minimum = false;
        }
    }

    if let Some(Some(right_neighbor)) = neighborhood.get((1, 2)) {
        if right_neighbor < &center {
            minimum = false;
        }
    }

    if let Some(Some(down_neighbor)) = neighborhood.get((2, 1)) {
        if down_neighbor < &center {
            minimum = false;
        }
    }

    (center != 9) && minimum
}

struct GridMinima {
    grid: Array2<Option<usize>>,
    minima: Array2<bool>,
}

impl GridMinima {
    pub fn new(g: &Array2<Option<usize>>) -> Self {
        let grid = g
            .slice(s![1..g.shape()[0] - 1, 1..g.shape()[1] - 1])
            .to_owned();
        let minima = Array::from_shape_fn(grid.dim(), |(i, j)| {
            is_minimum(g.slice(s![i..i + 3, j..j + 3]))
        });
        Self { grid, minima }
    }
}

fn part1(grid: &Array2<Option<usize>>) -> usize {
    let mut g = GridMinima::new(grid);
    g.grid.zip_mut_with(&g.minima, |grid, minimum| {
        if *minimum {
            *grid = Some(grid.unwrap() + 1);
        } else {
            *grid = None;
        }
    });
    g.grid.iter().flatten().sum()
}

fn flood_fill(grid: &mut Array2<bool>, node: (usize, usize)) -> usize {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back(node);
    let mut sum = 0;
    while !queue.is_empty() {
        let (i, j) = queue.pop_front().unwrap();
        let n = grid.get_mut((i, j));
        if let Some(a) = n {
            if *a {
                *a = false;
                sum += 1;
                if i < grid.dim().0 {
                    queue.push_back((i + 1, j));
                }
                if i > 0 {
                    queue.push_back((i - 1, j));
                }
                if j < grid.dim().1 {
                    queue.push_back((i, j + 1));
                }
                if j > 0 {
                    queue.push_back((i, j - 1));
                }
            }
        }
    }
    sum
}

fn part2(grid: &Array2<Option<usize>>) -> usize {
    let g = GridMinima::new(grid);
    let mut caves = Array2::from_shape_fn(g.grid.dim(), |(i, j)| {
        g.grid.get((i, j)).unwrap_or(&Some(0)).unwrap() != 9
    });

    let mut minima = Vec::new();
    for (i, row) in g.minima.rows().into_iter().enumerate() {
        row.iter()
            .enumerate()
            .filter(|(_, minimum)| **minimum)
            .for_each(|(j, _)| minima.push((i, j)));
    }
    let mut basins: Vec<usize> = minima.iter().map(|m| flood_fill(&mut caves, *m)).collect();
    basins.sort_unstable();
    basins.iter().rev().take(3).product()
}

fn parse_input(input: &[String]) -> Array2<Option<usize>> {
    let mut grid = Array2::<Option<usize>>::default((input.len() + 2, input[0].len() + 2));

    let v = input
        .iter()
        .map(|x| {
            x.chars()
                .map(|c| Some(c.to_digit(10).unwrap() as usize))
                .collect::<Vec<Option<usize>>>()
        })
        .flatten()
        .collect();

    let mut grid_slice = grid.slice_mut(s![1..input.len() + 1, 1..input[0].len() + 1]);
    grid_slice.assign(&Array2::from_shape_vec((input.len(), input[0].len()), v).unwrap());
    grid
}

pub fn run(lines: &[String]) {
    let input = parse_input(lines);
    run_parts!(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> Vec<String> {
        vec![
            "2199943210".to_string(),
            "3987894921".to_string(),
            "9856789892".to_string(),
            "8767896789".to_string(),
            "9899965678".to_string(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(&get_test_input())), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(&get_test_input())), 1134);
    }
}
