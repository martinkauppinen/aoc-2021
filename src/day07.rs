fn part1(positions: &[isize]) -> isize {
    let mut ps = positions.to_owned();
    ps.sort_unstable();
    let median = ps[ps.len() / 2];
    positions.iter().map(|x| (x - median).abs()).sum()
}

fn part2(positions: &[isize]) -> isize {
    let gauss = |x: isize| x * (x + 1) / 2;
    let max = positions.iter().max().unwrap();
    let mut best = isize::MAX;
    for i in 0..=*max {
        let sum = positions.iter().map(|x| gauss((*x - i).abs())).sum();
        if sum < best {
            best = sum;
        }
    }
    best
}

fn parse_input(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(|x| x.parse::<isize>().unwrap())
        .collect()
}

pub fn run(lines: &[String]) {
    let input = parse_input(&lines[0]);
    run_parts!(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> String {
        "16,1,2,0,4,2,7,1,2,14".to_string()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(&get_test_input())), 37);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(&get_test_input())), 168);
    }
}
