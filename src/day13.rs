use std::{fs, ops::BitOr, str::FromStr, string::ParseError};

use ndarray::{Array2, ArrayView, Axis, Slice};

struct Coord {
    x: usize,
    y: usize,
}

impl FromStr for Coord {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coord: Vec<&str> = s.split(',').collect();
        Ok(Coord {
            x: coord[0].parse().unwrap(),
            y: coord[1].parse().unwrap(),
        })
    }
}

fn array_from_coords(coords: &[Coord]) -> Array2<usize> {
    let x_max = coords.iter().max_by(|c1, c2| c1.x.cmp(&c2.x)).unwrap().x + 1;
    let y_max = coords.iter().max_by(|c1, c2| c1.y.cmp(&c2.y)).unwrap().y + 1;
    let mut arr = Array2::from_shape_fn((y_max, x_max), |(_, _)| 0);
    coords.iter().for_each(|c| {
        let elem = arr.get_mut((c.y, c.x)).unwrap();
        *elem = 1;
    });
    arr
}

fn perform_fold(arr: &mut Array2<usize>, fold: &Coord) {
    let axis;
    let index;
    if fold.x == 0 {
        axis = Axis(0);
        index = fold.y
    } else {
        axis = Axis(1);
        index = fold.x;
    }

    arr.remove_index(axis, index);
    let (a1, a2) = arr.view().split_at(axis, index);

    let stationary = a1.to_owned();
    let mut folded = a2.to_owned();

    while stationary.len_of(axis) != folded.len_of(axis) {
        if axis == Axis(0) {
            let new_row = vec![0; stationary.len_of(Axis(1))];
            folded.push_row(ArrayView::from(&new_row)).unwrap();
        } else {
            let new_col = vec![0; stationary.len_of(Axis(0))];
            folded.push_column(ArrayView::from(&new_col)).unwrap();
        }
    }

    *arr = stationary.bitor(folded.slice_axis(axis, Slice::new(0, None, -1)));
}

fn part1(input: &(Vec<Coord>, Vec<Coord>)) -> usize {
    let mut arr = array_from_coords(&input.0);
    perform_fold(&mut arr, &input.1[0]);
    arr.sum()
}

fn part2(input: &(Vec<Coord>, Vec<Coord>)) -> &str {
    let mut arr = array_from_coords(&input.0);
    for fold in &input.1 {
        perform_fold(&mut arr, fold);
    }

    let mut output: String = String::new();
    for row in arr.rows() {
        for col in row {
            if *col == 1 {
                output += "#";
            } else {
                output += " ";
            }
        }
        output += "\n";
    }
    fs::write("outputs/13.txt", output).expect("Unable to write output");

    "See outputs/13.txt"
}

fn parse_input(input: &[String]) -> (Vec<Coord>, Vec<Coord>) {
    let mut input_iter = input.split(|x| x.is_empty());
    let coords_str = input_iter.next().unwrap();
    let folds_str = input_iter.next().unwrap();

    let coords_vec = coords_str
        .iter()
        .map(|c| Coord::from_str(c).unwrap())
        .collect();

    let folds_vec = folds_str
        .iter()
        .map(|s| {
            let mut fold = s.split_whitespace().nth(2).unwrap().split('=');
            if fold.next().unwrap() == "x" {
                Coord {
                    x: fold.next().unwrap().parse().unwrap(),
                    y: 0,
                }
            } else {
                Coord {
                    x: 0,
                    y: fold.next().unwrap().parse().unwrap(),
                }
            }
        })
        .collect();

    (coords_vec, folds_vec)
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
            "6,10".to_string(),
            "0,14".to_string(),
            "9,10".to_string(),
            "0,3".to_string(),
            "10,4".to_string(),
            "4,11".to_string(),
            "6,0".to_string(),
            "6,12".to_string(),
            "4,1".to_string(),
            "0,13".to_string(),
            "10,12".to_string(),
            "3,4".to_string(),
            "3,0".to_string(),
            "8,4".to_string(),
            "1,10".to_string(),
            "2,14".to_string(),
            "8,10".to_string(),
            "9,0".to_string(),
            "".to_string(),
            "fold along y=7".to_string(),
            "fold along x=5".to_string(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(&get_test_input())), 17);
    }

    #[test]
    fn test_part2() {
        part2(&parse_input(&get_test_input()));
    }
}
