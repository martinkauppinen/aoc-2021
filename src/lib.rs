pub fn input_lines(filename: &str) -> Vec<String> {
    std::fs::read_to_string(filename)
        .expect("file not found")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<String>>()
}
