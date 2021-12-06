#[macro_use]
mod lib;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    let start = std::time::Instant::now();
    println!("━━━━┯{:━<60}┯{:━<10}", "", "");
    println!("Day │{:^60}│{:^10}", "Answer", "Time");
    day!(day01);
    day!(day02);
    day!(day03);
    day!(day04);
    day!(day05);
    day!(day06);
    let time = start.elapsed();
    println!("━━━━┷{:━<60}┷{:━<10}", "", "");
    println!("\nTotal time elapsed: {:?}", time);
}
