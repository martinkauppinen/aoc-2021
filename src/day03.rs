use crate::lib::input_lines;

fn strings_to_nums(lines: &[String]) -> Vec<u32> {
    lines
        .iter()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect()
}

fn most_common_bits(nums: &[u32], num_bits: usize) -> Vec<u32> {
    let mut bit_sums = vec![0; num_bits];

    nums.iter().fold(&mut bit_sums, |acc, x| {
        for (i, item) in acc.iter_mut().enumerate().take(num_bits) {
            *item += (x >> (num_bits - 1 - i)) & 1;
        }
        acc
    });

    for sum in &mut bit_sums {
        *sum = if *sum as usize >= (nums.len() + 1) / 2 {
            // Ceiling rounding
            1
        } else {
            0
        };
    }

    bit_sums
}

fn bit_vec_to_num(bits: Vec<u32>) -> u32 {
    bits.iter()
        .rev()
        .enumerate()
        .rev()
        .fold(0, |acc, (p, bit)| acc + *bit as u32 * 2u32.pow(p as u32))
}

fn iterative_filter(nums: &mut Vec<u32>, num_bits: usize, keep_equal: bool) -> u32 {
    let mut bit = num_bits - 1;
    let mut common = bit_vec_to_num(most_common_bits(nums, num_bits));

    loop {
        nums.retain(|x| ((x >> bit) & 1 == (common >> bit) & 1) == keep_equal);
        if nums.len() == 1 {
            break;
        }
        common = bit_vec_to_num(most_common_bits(nums, num_bits));
        bit -= 1;
    }

    nums[0]
}

fn part1(lines: &[String]) -> u32 {
    let num_bits = lines[0].len();
    let mask = 2u32.pow(num_bits as u32) - 1;
    let gamma = bit_vec_to_num(most_common_bits(
        &strings_to_nums(&lines.to_vec()),
        num_bits,
    ));
    let epsilon = !gamma & mask;
    gamma * epsilon
}

fn part2(lines: &[String]) -> u32 {
    let num_bits = lines[0].len();
    let oxygen_generator_rating =
        iterative_filter(&mut strings_to_nums(&lines.to_vec()), num_bits, true);
    let co2_scrubber_rating =
        iterative_filter(&mut strings_to_nums(&lines.to_vec()), num_bits, false);
    oxygen_generator_rating * co2_scrubber_rating
}

pub fn run() {
    let lines = input_lines("inputs/03.txt");

    println!("[Day 03] Part 1: {}", part1(&lines));
    println!("[Day 03] Part 2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> Vec<String> {
        vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&get_test_input()), 198);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_test_input()), 230);
    }
}
