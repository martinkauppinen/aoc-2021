fn dfs(
    cave: &str,
    connections: &[Vec<&str>],
    visited_caves: &mut Vec<String>,
    twice: Option<&String>,
) -> usize {
    if cave == "end" {
        return 1;
    }

    let mut sum = 0;

    if cave.to_lowercase() == *cave {
        visited_caves.push(cave.to_string());
        visited_caves.sort_unstable();
    }

    let connected_caves = connections
        .iter()
        .filter(|x| x.contains(&cave))
        .map(|x| x.iter().find(|y| *y != &cave).unwrap().to_string())
        .collect::<Vec<_>>();

    for other_cave in connected_caves.iter() {
        if !visited_caves.contains(other_cave) {
            sum += dfs(other_cave, connections, visited_caves, twice);
        }
    }

    if twice == Some(&"".to_string()) {
        for other_cave in connected_caves.iter() {
            if visited_caves.contains(other_cave) && other_cave != "start" {
                sum += dfs(other_cave, connections, visited_caves, Some(other_cave));
            }
        }
    }

    if twice != Some(&cave.to_string()) {
        visited_caves.retain(|x| x != cave);
    }

    visited_caves.dedup();
    sum
}

fn part1(connections: &[Vec<&str>]) -> usize {
    dfs(&"start".to_string(), connections, &mut Vec::new(), None)
}

fn part2(connections: &[Vec<&str>]) -> usize {
    dfs(
        &"start".to_string(),
        connections,
        &mut Vec::new(),
        Some(&"".to_string()),
    )
}

fn parse_input(lines: &[String]) -> Vec<Vec<&str>> {
    lines
        .iter()
        .map(|x| x.split('-').collect::<Vec<_>>())
        .collect()
}

pub fn run(lines: &[String]) {
    let input = parse_input(lines);
    run_parts!(input);
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_test_input_small() -> Vec<String> {
        vec![
            "start-A".to_string(),
            "start-b".to_string(),
            "A-c".to_string(),
            "A-b".to_string(),
            "b-d".to_string(),
            "A-end".to_string(),
            "b-end".to_string(),
        ]
    }

    fn get_test_input_medium() -> Vec<String> {
        vec![
            "dc-end".to_string(),
            "HN-start".to_string(),
            "start-kj".to_string(),
            "dc-start".to_string(),
            "dc-HN".to_string(),
            "LN-dc".to_string(),
            "HN-end".to_string(),
            "kj-sa".to_string(),
            "kj-HN".to_string(),
            "kj-dc".to_string(),
        ]
    }

    fn get_test_input_large() -> Vec<String> {
        vec![
            "fs-end".to_string(),
            "he-DX".to_string(),
            "fs-he".to_string(),
            "start-DX".to_string(),
            "pj-DX".to_string(),
            "end-zg".to_string(),
            "zg-sl".to_string(),
            "zg-pj".to_string(),
            "pj-he".to_string(),
            "RW-he".to_string(),
            "fs-DX".to_string(),
            "pj-RW".to_string(),
            "zg-RW".to_string(),
            "start-pj".to_string(),
            "he-WI".to_string(),
            "zg-he".to_string(),
            "pj-fs".to_string(),
            "start-RW".to_string(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(&get_test_input_small())), 10);
        assert_eq!(part1(&parse_input(&get_test_input_medium())), 19);
        assert_eq!(part1(&parse_input(&get_test_input_large())), 226);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(&get_test_input_small())), 36);
        assert_eq!(part2(&parse_input(&get_test_input_medium())), 103);
        assert_eq!(part2(&parse_input(&get_test_input_large())), 3509);
    }
}
