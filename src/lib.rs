use std::fmt::Display;
use std::time::Duration;

pub fn input_lines(filename: &str) -> Vec<String> {
    std::fs::read_to_string(filename)
        .expect("file not found")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<String>>()
}

pub struct MyDuration(pub Duration);

impl Display for MyDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.as_secs() == 0 {
            if self.0.as_millis() == 0 {
                if self.0.as_micros() == 0 {
                    write!(f, "{:6} ns", self.0.subsec_nanos())
                } else {
                    let whole = self.0.subsec_micros();
                    let part = (self.0.subsec_nanos() % 1000) / 10;
                    write!(f, "{:3}.{:02} µs", whole, part)
                }
            } else {
                let whole = self.0.subsec_millis();
                let part = (self.0.subsec_micros() % 1000) / 10;
                write!(f, "{:3}.{:02} ms", whole, part)
            }
        } else if self.0.as_secs() < 1000 {
            write!(f, "{:>6.2}  s", self.0.as_secs_f64())
        } else {
            write!(f, "many    s")
        }
    }
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

#[macro_export]
macro_rules! run_parts {
    ($input:ident) => {
        let module = module_path!().split("::").last().unwrap()[3..]
            .parse::<u8>()
            .unwrap();

        let part1_start = std::time::Instant::now();
        let part1_answer = part1(&$input);
        let part1_time = crate::lib::MyDuration(part1_start.elapsed());
        let part2_start = std::time::Instant::now();
        let part2_answer = part2(&$input);
        let part2_time = crate::lib::MyDuration(part2_start.elapsed());

        println!("────┼{:─<60}┼{:─<10}", "", "");
        println!(
            "{:3} │ Part 1: {:50} │ {} ",
            module, part1_answer, part1_time
        );
        println!("{:3} │ Part 2: {:50} │ {} ", "", part2_answer, part2_time);
    };
}
