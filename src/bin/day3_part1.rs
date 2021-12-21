use itertools::Itertools;
use std::fs;

fn main() {
    let filename = "day3_input.txt";
    let diagnostic_report = fs::read_to_string(filename).expect("File read error");

    println!("{}", diagnostic_report);

    let result = calculate_power_consumption(&diagnostic_report);

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

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let input = indoc! {"00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010"};

        let result = calculate_power_consumption(input);
        assert_eq!(198, result);
    }
}
