fn part1(s: &[Vec<&str>]) -> usize {
    let digits: Vec<&str> = s.iter().map(|x| x[1]).collect();
    let mut sum = 0;
    digits.join(" ").split_whitespace().for_each(|x| {
        if x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7 {
            sum += 1;
        }
    });
    sum
}

fn count_intersects(a: &str, b: &str) -> usize {
    a.chars().filter(|c| b.contains(*c)).count()
}

fn sorted_equal(a: &str, b: &str) -> bool {
    let mut a_sorted: Vec<char> = a.chars().collect();
    let mut b_sorted: Vec<char> = b.chars().collect();

    a_sorted.sort_unstable();
    b_sorted.sort_unstable();

    a_sorted == b_sorted
}

fn parse_one_entry(entry: Vec<&str>) -> usize {
    let e = entry.to_owned();
    let mut signals: Vec<&str> = e[0].split_whitespace().collect();
    let output: Vec<&str> = e[1].split_whitespace().collect();

    signals.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());

    let mut d1 = "";
    let mut d4 = "";
    let mut d7 = "";
    let mut d8 = "";
    let mut unknowns = signals[3..9].to_vec();
    if let [one, seven, four, .., eight] = signals.as_slice() {
        d1 = one;
        d4 = four;
        d7 = seven;
        d8 = eight;
    }

    // The only digit that 4 shares all segments with is 9
    let d9 = unknowns
        .iter()
        .find(|x| count_intersects(d4, x) == 4)
        .unwrap()
        .to_owned();
    unknowns.retain(|x| *x != d9);

    // The only digit now that shares 2 segments with 4 is 2
    let d2 = unknowns
        .iter()
        .find(|x| count_intersects(d4, x) == 2)
        .unwrap()
        .to_owned();
    unknowns.retain(|x| *x != d2);

    // The only digit now that shares 3 segments with 2 is 5
    let d5 = unknowns
        .iter()
        .find(|x| count_intersects(d2, x) == 3)
        .unwrap()
        .to_owned();
    unknowns.retain(|x| *x != d5);

    // 3 is the only remaining digit with 5 segments
    let d3 = unknowns.iter().find(|x| x.len() == 5).unwrap().to_owned();
    unknowns.retain(|x| *x != d3);

    // The only digit that 5 shares all segments with is 6
    let d6 = unknowns
        .iter()
        .find(|x| count_intersects(d5, x) == 5)
        .unwrap()
        .to_owned();
    unknowns.retain(|x| *x != d6);

    // 0 is the final digit
    let d0 = unknowns.pop().unwrap();

    let digits = [d0, d1, d2, d3, d4, d5, d6, d7, d8, d9];

    let mut ret = 0;

    ret += digits
        .iter()
        .position(|x| sorted_equal(*x, output[0]))
        .unwrap()
        * 1000;
    ret += digits
        .iter()
        .position(|x| sorted_equal(*x, output[1]))
        .unwrap()
        * 100;
    ret += digits
        .iter()
        .position(|x| sorted_equal(*x, output[2]))
        .unwrap()
        * 10;
    ret += digits
        .iter()
        .position(|x| sorted_equal(*x, output[3]))
        .unwrap();

    ret
}

fn part2(s: &[Vec<&str>]) -> usize {
    s.iter().map(|x| parse_one_entry(x.to_vec())).sum()
}

fn parse_input(input: &[String]) -> Vec<Vec<&str>> {
    input.iter().map(|x| x.split(" | ").collect()).collect()
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
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".to_string(),
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc".to_string(),
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg".to_string(),
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb".to_string(),
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea".to_string(),
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb".to_string(),
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe".to_string(),
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef".to_string(),
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb".to_string(),
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".to_string(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(&get_test_input())), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(&get_test_input())), 61229);
    }
}
