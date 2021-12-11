#[macro_use]
mod lib;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

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
    day!(day07);
    day!(day08);
    day!(day09);
    day!(day10);
    day!(day11);
    let time = start.elapsed();
    println!("━━━━┷{:━<60}┷{:━<10}", "", "");
    println!("\nTotal time elapsed: {:?}", time);
}
