fn sequence_increases(sequence: Vec<isize>) -> usize {
    let pairs = sequence.as_slice().windows(2);
    let differences = pairs.map(|p| p[1] - p[0]);
    differences.filter(|&x| x > 0).count()
}

fn part1(numbers: &[isize]) -> usize {
    sequence_increases(numbers.to_vec())
}

fn part2(numbers: &[isize]) -> usize {
    let windows = numbers.windows(3);
    let sums = windows.map(|w| w[0] + w[1] + w[2]).collect();
    sequence_increases(sums)
}

pub fn run(lines: &[String]) {
    let numbers: Vec<isize> = lines
        .iter()
        .map(|line| line.parse::<isize>().unwrap())
        .collect();

    run_parts!(numbers);
}

#[cfg(test)]
mod tests {
    use super::*;
    const NUMBERS: [isize; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn test_part1() {
        assert_eq!(part1(&NUMBERS), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&NUMBERS), 5);
    }
}
