fn step(states: &mut Vec<usize>) {
    states.rotate_left(1);
    states[6] += states[8];
}

fn simulate(initial_state: &[usize], days: usize) -> usize {
    let mut states = initial_state.to_owned();
    for _ in 0..days {
        step(&mut states);
    }
    states.iter().sum()
}

fn part1(initial_state: &[usize]) -> usize {
    simulate(initial_state, 80)
}

fn part2(initial_state: &[usize]) -> usize {
    simulate(initial_state, 256)
}

fn parse_input(s: &str) -> Vec<usize> {
    let mut state: Vec<usize> = Vec::new();
    s.split(',').for_each(|x| state.push(x.parse().unwrap()));
    let mut states: Vec<usize> = vec![0; 9];
    state.iter().for_each(|x| states[*x] += 1);
    states
}

pub fn run(lines: &[String]) {
    let init_state = parse_input(&lines[0]);
    run_parts!(init_state);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> String {
        "3,4,3,1,2".to_string()
    }

    #[test]
    fn test_part1() {
        assert_eq!(simulate(&parse_input(&get_test_input()), 18), 26);
        assert_eq!(simulate(&parse_input(&get_test_input()), 80), 5934);
    }

    #[test]
    fn test_part2() {
        assert_eq!(simulate(&parse_input(&get_test_input()), 256), 26984457539);
    }
}
