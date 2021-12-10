fn matching_delim(c: char) -> char {
    match c {
        '(' => ')',
        '<' => '>',
        '{' => '}',
        '[' => ']',
        ')' => '(',
        '>' => '<',
        '}' => '{',
        ']' => '[',
        _ => panic!(),
    }
}

fn parse_brackets(s: &str) -> (Option<char>, Option<Vec<char>>) {
    let mut stack: Vec<char> = Vec::new();
    let opening = "(<{[";
    for c in s.chars() {
        if opening.contains(c) {
            stack.push(c);
            continue;
        }

        if stack.is_empty() || c != matching_delim(stack.pop().unwrap()) {
            return (Some(c), Some(stack));
        }
    }
    if stack.is_empty() {
        return (None, None);
    }

    (None, Some(stack))
}

fn part1(input: &[String]) -> usize {
    input
        .iter()
        .map(|x| parse_brackets(x).0)
        .filter(|x| x.is_some())
        .flatten()
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        })
        .sum()
}

fn part2(input: &[String]) -> usize {
    let mut complete_scores = input
        .iter()
        .map(|x| parse_brackets(x))
        .filter(|x| x.0.is_none())
        .filter(|x| x.1.is_some())
        .map(|x| x.1)
        .flatten()
        .map(|stack| {
            stack.iter().rev().fold(0, |acc, c| match c {
                '(' => acc * 5 + 1,
                '[' => acc * 5 + 2,
                '{' => acc * 5 + 3,
                '<' => acc * 5 + 4,
                _ => unreachable!(),
            })
        })
        .collect::<Vec<usize>>();
    complete_scores.sort_unstable();
    complete_scores[complete_scores.len() / 2]
}

pub fn run(lines: &[String]) {
    run_parts!(lines);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> Vec<String> {
        vec![
            "[({(<(())[]>[[{[]{<()<>>".to_string(),
            "[(()[<>])]({[<{<<[]>>(".to_string(),
            "{([(<{}[<>[]}>{[]{[(<()>".to_string(),
            "(((({<>}<{<{<>}{[]{[]{}".to_string(),
            "[[<[([]))<([[{}[[()]]]".to_string(),
            "[{[{({}]{}}([{[{{{}}([]".to_string(),
            "{<[[]]>}<{[{[{[]{()[[[]".to_string(),
            "[<(<(<(<{}))><([]([]()".to_string(),
            "<{([([[(<>()){}]>(<<{{".to_string(),
            "<{([{{}}[<[[[<>{}]]]>[]]".to_string(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&get_test_input()), 26397);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_test_input()), 288957);
    }
}
