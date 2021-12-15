// Gotta love the Rust docs
// https://doc.rust-lang.org/std/collections/binary_heap/index.html
// Entire Dijkstra implementation

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Edge {
    node: usize,
    cost: usize,
}
fn shortest_path(adj_list: &[Vec<Edge>], start: usize, goal: usize) -> Option<usize> {
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }

        if cost > dist[position] {
            continue;
        }

        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }
    None
}

fn part1(lines: &[String]) -> usize {
    let (graph, end) = grid_to_graph(parse_input(lines));
    shortest_path(&graph, 0, end).unwrap()
}

fn concat_grids_horizontal(g1: &[Vec<usize>], g2: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut ret = Vec::new();

    for (i, row) in g1.iter().enumerate() {
        ret.push(vec![row.clone(), g2[i].clone()].concat());
    }

    ret
}

fn part2(lines: &[String]) -> usize {
    let grid = parse_input(lines);

    let inc = |g: &Vec<Vec<usize>>, x: usize| -> Vec<Vec<usize>> {
        let mut ret = g.clone();
        ret.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|val| {
                *val += x;
                if *val > 9 {
                    *val = (*val % 10) + 1;
                }
            });
        });
        ret
    };

    let mut big_grid_row = grid.clone();

    for i in 1..5 {
        big_grid_row = concat_grids_horizontal(&big_grid_row, &inc(&grid, i));
    }

    let mut big_grid = big_grid_row.clone();

    for i in 1..5 {
        big_grid.append(&mut inc(&big_grid_row, i).clone());
    }

    let (graph, end) = grid_to_graph(big_grid);

    shortest_path(&graph, 0, end).unwrap()
}

fn grid_to_graph(grid: Vec<Vec<usize>>) -> (Vec<Vec<Edge>>, usize) {
    let last_index = grid.len() * grid[0].len() - 1;
    let mut graph = Vec::new();

    for (i, row) in grid.iter().enumerate() {
        let mut row_neighbors = Vec::new();
        for (j, _) in row.iter().enumerate() {
            let mut neighbors = Vec::new();

            if i > 0 {
                neighbors.push(Edge {
                    node: (i - 1) * row.len() + j,
                    cost: grid[i - 1][j],
                });
            }

            if j > 0 {
                neighbors.push(Edge {
                    node: i * row.len() + (j - 1),
                    cost: grid[i][j - 1],
                });
            }

            if i < row.len() - 1 {
                neighbors.push(Edge {
                    node: (i + 1) * row.len() + j,
                    cost: grid[i + 1][j],
                });
            }

            if j < grid.len() - 1 {
                neighbors.push(Edge {
                    node: i * row.len() + (j + 1),
                    cost: grid[i][j + 1],
                });
            }

            row_neighbors.push(neighbors);
        }
        graph.append(&mut row_neighbors);
    }

    (graph, last_index)
}

fn parse_input(input: &[String]) -> Vec<Vec<usize>> {
    input
        .iter()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn run(lines: &[String]) {
    run_parts!(lines);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> Vec<String> {
        vec![
            "1163751742".to_string(),
            "1381373672".to_string(),
            "2136511328".to_string(),
            "3694931569".to_string(),
            "7463417111".to_string(),
            "1319128137".to_string(),
            "1359912421".to_string(),
            "3125421639".to_string(),
            "1293138521".to_string(),
            "2311944581".to_string(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&get_test_input()), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_test_input()), 315);
    }
}
