use std::collections::HashMap;

type Elements = HashMap<char, usize>;
type Rules = HashMap<String, String>;
type Polymer = HashMap<String, usize>;

fn step(polymer: &mut Polymer, rules: &Rules, elements: &mut Elements) {
    let pairs = polymer.keys().cloned().collect::<Vec<_>>();
    let mut new_pairs = Polymer::new();

    for pair in pairs {
        let count = *polymer.get(&pair).unwrap();
        polymer.insert(pair.clone(), 0);

        let insertion = rules.get(&pair).unwrap().clone().chars().last().unwrap();
        *elements.entry(insertion).or_default() += count;

        let mut first_char = pair.clone();
        let second_char = first_char.split_off(1);
        let new_pair1 = first_char + &rules.get(&pair).unwrap().clone();
        let new_pair2 = rules.get(&pair).unwrap().clone() + &second_char;
        *new_pairs.entry(new_pair1).or_default() += count;
        *new_pairs.entry(new_pair2).or_default() += count;
    }

    for (k, v) in new_pairs {
        polymer.insert(k, v);
    }
}

fn run_steps(steps: usize, p: &mut Polymer, r: &Rules, e: &mut Elements) -> usize {
    for _ in 0..steps {
        step(p, r, e);
    }

    let mut least = usize::MAX;
    let mut most = usize::MIN;

    for v in e.values() {
        if *v < least {
            least = *v;
        }
        if *v > most {
            most = *v;
        }
    }

    most - least
}

fn part1(lines: &[String]) -> usize {
    let (mut polymer, rules, mut elements) = parse_lines2(lines);
    run_steps(10, &mut polymer, &rules, &mut elements)
}

fn part2(lines: &[String]) -> usize {
    let (mut polymer, rules, mut elements) = parse_lines2(lines);
    run_steps(40, &mut polymer, &rules, &mut elements)
}

fn parse_lines2(lines: &[String]) -> (Polymer, Rules, Elements) {
    let mut input_iter = lines.split(|x| x.is_empty());

    let polymer_template = input_iter.next().unwrap()[0].clone();
    let pair_insertions_input = input_iter.next().unwrap();

    let mut polymer = Polymer::new();
    let mut elements = Elements::new();

    for pair in polymer_template.chars().collect::<Vec<_>>().windows(2) {
        let pair_str = pair.iter().collect();
        *polymer.entry(pair_str).or_default() += 1;
    }

    for c in polymer_template.chars() {
        *elements.entry(c).or_default() += 1;
    }

    let mut pair_insertion_rules = Rules::new();

    pair_insertions_input.iter().for_each(|line| {
        let rule_str = line.split(" -> ").collect::<Vec<_>>();
        *pair_insertion_rules
            .entry(rule_str[0].to_string())
            .or_default() = rule_str[1].to_string();
    });

    (polymer, pair_insertion_rules, elements)
}

pub fn run(lines: &[String]) {
    run_parts!(lines);
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_test_input() -> Vec<String> {
        vec![
            "NNCB".to_string(),
            "".to_string(),
            "CH -> B".to_string(),
            "HH -> N".to_string(),
            "CB -> H".to_string(),
            "NH -> C".to_string(),
            "HB -> C".to_string(),
            "HC -> B".to_string(),
            "HN -> C".to_string(),
            "NN -> C".to_string(),
            "BH -> H".to_string(),
            "NC -> B".to_string(),
            "NB -> B".to_string(),
            "BN -> B".to_string(),
            "BB -> N".to_string(),
            "BC -> B".to_string(),
            "CC -> N".to_string(),
            "CN -> C".to_string(),
        ]
    }

    #[test]
    fn test_steps() {
        let (mut polymer, rules, mut elements) = parse_lines2(&get_test_input());

        let step_0 = "NNCB";
        let step_1 = "NCNBCHB";
        let step_2 = "NBCCNBBBCBHCB";
        let step_3 = "NBBBCNCCNBBNBNBBCHBHHBCHB";
        let step_4 = "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB";

        assert_eq!(
            elements[&'N'],
            step_0.chars().filter(|c| *c == 'N').count(),
            "[0 steps] wrong number of N"
        );
        assert_eq!(
            elements[&'C'],
            step_0.chars().filter(|c| *c == 'C').count(),
            "[0 steps] wrong number of C"
        );
        assert_eq!(
            elements[&'B'],
            step_0.chars().filter(|c| *c == 'B').count(),
            "[0 steps] wrong number of B"
        );
        for w in step_0.chars().collect::<Vec<_>>().windows(2) {
            assert!(
                polymer.contains_key(&w.iter().collect::<String>()),
                "[0 steps] Does not contain {:?}",
                w
            );
            assert_ne!(
                polymer[&w.iter().collect::<String>()],
                0,
                "[0 steps] Key had 0 value: {:?}",
                w
            );
        }

        step(&mut polymer, &rules, &mut elements);

        assert_eq!(
            elements[&'N'],
            step_1.chars().filter(|c| *c == 'N').count(),
            "[1 step] wrong number of N"
        );
        assert_eq!(
            elements[&'C'],
            step_1.chars().filter(|c| *c == 'C').count(),
            "[1 step] wrong number of C"
        );
        assert_eq!(
            elements[&'B'],
            step_1.chars().filter(|c| *c == 'B').count(),
            "[1 step] wrong number of B"
        );
        assert_eq!(
            elements[&'H'],
            step_1.chars().filter(|c| *c == 'H').count(),
            "[1 step] wrong number of H"
        );
        for w in step_1.chars().collect::<Vec<_>>().windows(2) {
            assert!(
                polymer.contains_key(&w.iter().collect::<String>()),
                "[1 step] Does not contain {:?}",
                w
            );
            assert_ne!(
                polymer[&w.iter().collect::<String>()],
                0,
                "[1 step] Key had 0 value: {:?}",
                w
            );
        }

        step(&mut polymer, &rules, &mut elements);

        assert_eq!(
            elements[&'N'],
            step_2.chars().filter(|c| *c == 'N').count(),
            "[2 steps] wrong number of N"
        );
        assert_eq!(
            elements[&'C'],
            step_2.chars().filter(|c| *c == 'C').count(),
            "[2 steps] wrong number of C"
        );
        assert_eq!(
            elements[&'B'],
            step_2.chars().filter(|c| *c == 'B').count(),
            "[2 steps] wrong number of B"
        );
        assert_eq!(
            elements[&'H'],
            step_2.chars().filter(|c| *c == 'H').count(),
            "[2 steps] wrong number of H"
        );
        for w in step_2.chars().collect::<Vec<_>>().windows(2) {
            assert!(
                polymer.contains_key(&w.iter().collect::<String>()),
                "[2 steps] Does not contain {:?}",
                w
            );
            assert_ne!(
                polymer[&w.iter().collect::<String>()],
                0,
                "[2 steps] Key had 0 value: {:?}",
                w
            );
        }

        step(&mut polymer, &rules, &mut elements);

        assert_eq!(
            elements[&'N'],
            step_3.chars().filter(|c| *c == 'N').count(),
            "[3 steps] wrong number of N"
        );
        assert_eq!(
            elements[&'C'],
            step_3.chars().filter(|c| *c == 'C').count(),
            "[3 steps] wrong number of C"
        );
        assert_eq!(
            elements[&'B'],
            step_3.chars().filter(|c| *c == 'B').count(),
            "[3 steps] wrong number of B"
        );
        assert_eq!(
            elements[&'H'],
            step_3.chars().filter(|c| *c == 'H').count(),
            "[3 steps] wrong number of H"
        );
        for w in step_3.chars().collect::<Vec<_>>().windows(2) {
            assert!(
                polymer.contains_key(&w.iter().collect::<String>()),
                "[3 steps] Does not contain {:?}",
                w
            );
            assert_ne!(
                polymer[&w.iter().collect::<String>()],
                0,
                "[3 steps] Key had 0 value: {:?}",
                w
            );
        }

        step(&mut polymer, &rules, &mut elements);

        assert_eq!(
            elements[&'N'],
            step_4.chars().filter(|c| *c == 'N').count(),
            "[4 steps] wrong number of N"
        );
        assert_eq!(
            elements[&'C'],
            step_4.chars().filter(|c| *c == 'C').count(),
            "[4 steps] wrong number of C"
        );
        assert_eq!(
            elements[&'B'],
            step_4.chars().filter(|c| *c == 'B').count(),
            "[4 steps] wrong number of B"
        );
        assert_eq!(
            elements[&'H'],
            step_4.chars().filter(|c| *c == 'H').count(),
            "[4 steps] wrong number of H"
        );
        for w in step_4.chars().collect::<Vec<_>>().windows(2) {
            assert!(
                polymer.contains_key(&w.iter().collect::<String>()),
                "[4 steps] Does not contain {:?}",
                w
            );
            assert_ne!(
                polymer[&w.iter().collect::<String>()],
                0,
                "[4 steps] Key had 0 value: {:?}",
                w
            );
        }
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&get_test_input()), 1588);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_test_input()), 2188189693529);
    }
}
