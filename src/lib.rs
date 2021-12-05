pub fn input_lines(filename: &str) -> Vec<String> {
    std::fs::read_to_string(filename)
        .expect("file not found")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<String>>()
}

#[macro_export]
macro_rules! day {
    ($x:ident) => {
        let day_number = &stringify!($x)[3..5];
        $x::run(&lib::input_lines(
            &format!("inputs/{}.txt", day_number).to_string(),
        ))
    };
}
