use std::cmp::{max, min};
use std::num::ParseIntError;
use std::ops::{Add, Mul, Sub};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

struct Grid {
    grid: Vec<Vec<usize>>,
    intersections: usize,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            grid: vec![vec![0; 1000]; 1000],
            intersections: 0,
        }
    }

    fn insert_point(&mut self, p: Point) {
        if self.grid[p.x as usize][p.y as usize] == 1 {
            self.intersections += 1;
        }
        self.grid[p.x as usize][p.y as usize] += 1;
    }

    pub fn insert_line(&mut self, ps: (Point, Point)) {
        let diff = ps.1 - ps.0;
        let dir = Point {
            x: diff.x.signum(),
            y: diff.y.signum(),
        };
        if diff.x != 0 && straight_path(ps) {
            for i in min(ps.0.x, ps.1.x)..=max(ps.0.x, ps.1.x) {
                self.insert_point(Point { x: i, y: ps.0.y });
            }
        } else if diff.y != 0 && straight_path(ps) {
            for i in min(ps.0.y, ps.1.y)..=max(ps.0.y, ps.1.y) {
                self.insert_point(Point { x: ps.0.x, y: i });
            }
        } else {
            // Diagonal path
            for i in 0..=isize::abs(diff.x) {
                self.insert_point(ps.0 + dir * i);
            }
        }
    }

    pub fn count_intersections(&self) -> usize {
        self.intersections
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords = s.trim().split(',').collect::<Vec<&str>>();
        let x = isize::from_str(coords[0])?;
        let y = isize::from_str(coords[1])?;
        Ok(Point { x, y })
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<isize> for Point {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Point {
            x: rhs * self.x,
            y: rhs * self.y,
        }
    }
}

fn parse_lines(lines: &[String]) -> Vec<(Point, Point)> {
    let mut points: Vec<(Point, Point)> = Vec::new();
    lines.iter().for_each(|line| {
        let point_strs = line.split("->").collect::<Vec<&str>>();
        let point_pair = (
            Point::from_str(point_strs[0]).expect("failed to parse point"),
            Point::from_str(point_strs[1]).expect("failed to parse point"),
        );
        points.push(point_pair);
    });
    points
}

fn straight_path(ps: (Point, Point)) -> bool {
    ps.0.x == ps.1.x || ps.0.y == ps.1.y
}

fn count_points(ps: Vec<(Point, Point)>) -> usize {
    let mut grid = Grid::new();
    ps.iter().for_each(|&p| {
        grid.insert_line(p);
    });
    grid.count_intersections()
}

fn part1(lines: &[String]) -> usize {
    let mut ps = parse_lines(lines);
    ps.retain(|p| straight_path(*p));
    count_points(ps)
}

fn part2(lines: &[String]) -> usize {
    let ps = parse_lines(lines);
    count_points(ps)
}

pub fn run(lines: &[String]) {
    run_parts!(lines);
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_test_input() -> Vec<String> {
        vec![
            "0,9 -> 5,9".to_string(),
            "8,0 -> 0,8".to_string(),
            "9,4 -> 3,4".to_string(),
            "2,2 -> 2,1".to_string(),
            "7,0 -> 7,4".to_string(),
            "6,4 -> 2,0".to_string(),
            "0,9 -> 2,9".to_string(),
            "3,4 -> 1,4".to_string(),
            "0,0 -> 8,8".to_string(),
            "5,5 -> 8,2".to_string(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&get_test_input()), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_test_input()), 12);
    }
}
