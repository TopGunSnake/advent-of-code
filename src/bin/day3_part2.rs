use itertools::enumerate;
use itertools::Itertools;
use std::env;
use std::fs;

fn main() {
    let filename = "day3_input.txt";
    let diagnostic_report = fs::read_to_string(filename).expect("File read error");

    println!("{}", diagnostic_report);

    let result = calculate_oxygen_generator_rating(&diagnostic_report)
        * calculate_co2_scrubber_rating(&diagnostic_report);

    println!("Final result: {}", result);
}

fn calculate_power_consumption(report: &str) -> u32 {
    let data = report
        .lines()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect_vec();
    let line_width = report.lines().next().unwrap().len() as u32;
    let gamma_rate = get_gamma_rate(&data, line_width);
    let epsilon_rate = get_epsilon_rate(&data, line_width);

    gamma_rate * epsilon_rate
}

/// Converts by using least common bit values
fn get_epsilon_rate(report: &[u32], width: u32) -> u32 {
    let accumulator = accumulate_bit_counts(width, report);

    let mut result = 0u32;
    for (i, &bit) in accumulator.iter().enumerate() {
        if bit < 0 {
            result += 1 << i;
        }
    }

    result
}

/// Converts by using most common bit values
fn get_gamma_rate(report: &[u32], width: u32) -> u32 {
    let accumulator = accumulate_bit_counts(width, report);

    let mut result = 0u32;
    for (i, &bit) in accumulator.iter().enumerate() {
        if bit > 0 {
            result += 1 << i;
        }
    }

    result
}

/// Returns a vector of counts of 1 against 0
///
/// A field is positive if there were more 1's than 0's
/// Order is big-endian (Vec[0] indicates the 1's place of the binary)
fn accumulate_bit_counts(width: u32, report: &[u32]) -> Vec<i32> {
    let mut accumulator = Vec::from_iter(std::iter::repeat(0).take(width as usize));
    for line in report {
        for (i, bit) in accumulator.iter_mut().enumerate() {
            *bit += match line >> i & 1 {
                1 => 1,
                0 => -1,
                _ => panic!("Bad logic in match"),
            }
        }
    }
    accumulator
}

fn calculate_co2_scrubber_rating(report: &str) -> u32 {
    let mut data = report
        .lines()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect_vec();
    let line_width = report.lines().next().unwrap().len() as u32;

    for i in (0..line_width).rev() {
        let accumulator = accumulate_bit_counts(line_width, &data);

        let count = accumulator[i as usize];
        data.retain(|&x| {
            let bit = x >> i & 1;
            bit == if count < 0 { 1 } else { 0 }
        });
        debug_data(i as usize, &data);
        if data.len() == 1 {
            break;
        }
    }
    if data.len() != 1 {
        panic!("Bad reduction, should only have one result remaining!");
    } else {
        data[0]
    }
}

fn calculate_oxygen_generator_rating(report: &str) -> u32 {
    let mut data = report
        .lines()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect_vec();
    let line_width = report.lines().next().unwrap().len() as u32;

    for i in (0..line_width).rev() {
        let accumulator = accumulate_bit_counts(line_width, &data);
        let count = accumulator[i as usize];
        println!("Removing based on count: {:?} {}", accumulator, count);
        data.retain(|&x| {
            let bit = x >> i & 1;
            bit == (if count >= 0 { 1 } else { 0 })
        });
        debug_data(i as usize, &data);
        if data.len() == 1 {
            break;
        }
    }

    if data.len() != 1 {
        panic!("Bad reduction, should only have one result remaining!");
    } else {
        data[0]
    }
}

fn debug_data(i: usize, data: &Vec<u32>) {
    println!("bit {}", i);
    for item in data {
        println!("{:05b}", item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

        let oxygen = calculate_oxygen_generator_rating(input);
        assert_eq!(23, oxygen);
    }

    #[test]
    fn test_co2_example() {
        let input =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

        let co2 = calculate_co2_scrubber_rating(input);
        assert_eq!(10, co2);
    }
}
