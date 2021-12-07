use itertools::Itertools;
use std::env;
use std::fs;

fn main() {
    let filename = "day6_input.txt";
    let contents = fs::read_to_string(filename).expect("File read error");

    println!("{}", contents);

    let result_part1 = simulate_fish(&contents, 80);

    println!("Result ({} days): {}", 80, result_part1);

    let result_part2 = simulate_fish(&contents, 256);

    println!("Result ({} days): {}", 256, result_part2);
}

fn simulate_fish(input: &str, max_days: u32) -> u128 {
    let fish = input
        .split_terminator(',')
        .map(|s| s.parse::<u32>().unwrap());
    let mut groups = [0; 9];
    for f in fish {
        groups[f as usize] += 1;
    }

    for day in 0..max_days {
        let tmp = groups[0];
        groups.rotate_left(1);
        groups[6] += tmp;
        println!("Count after day {}: {:?}", day, groups);
    }

    groups.iter().sum::<_>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example_string = "3,4,3,1,2";
        println!("{}", example_string);
        let result = simulate_fish(example_string, 80);
        assert_eq!(5934, result);
    }

    #[test]
    fn test_example_part2() {
        let example_string = "3,4,3,1,2";
        println!("{}", example_string);
        let result = simulate_fish(example_string, 256);
        assert_eq!(26984457539, result);
    }
}
