use std::fs;

use itertools::Itertools;

fn main() {
    let filename = "day7_input.txt";
    let contents = fs::read_to_string(filename).expect("File read error");

    println!("{}", contents);

    let result = do_the_thing(&contents);

    println!("Result {}", result);
}

fn do_the_thing(input: &str) -> u32 {
    let crabs = input
        .split_terminator(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect_vec();

    let (mut lower, mut upper) = (0, crabs.iter().max().unwrap().to_owned());
    let func = |x| {
        crabs
            .iter()
            .map(|&c| (c as i32 - x as i32).abs())
            .sum::<i32>()
    };
    while lower != upper {
        let left = (lower + upper) / 2;
        let right = left + 1;

        let slope = func(left) - func(right);

        if slope < 0 {
            // right was greater than left, so we look left,
            upper = left;
        } else {
            // left was greater than right, so we look right,
            lower = right;
        }
    }
    func(lower) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "16,1,2,0,4,2,7,1,2,14";

        let result = do_the_thing(input);

        assert_eq!(37, result);
    }
}
