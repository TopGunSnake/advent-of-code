use itertools::Itertools;
use std::fs;

fn main() {
    let filename = "day1_input.txt";
    let contents = fs::read_to_string(filename).expect("File read error");

    println!("{}", contents);

    let count = count_increases(&contents);
    println!("Count: {}", count);
}

fn count_increases(contents: &str) -> i32 {
    let mut count = 0;
    for (s0, s1) in contents.lines().tuple_windows() {
        if (s1.parse::<i32>().unwrap()) > (s0.parse::<i32>().unwrap()) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::count_increases;
    use indoc::indoc;
    #[test]
    fn test_example() {
        let example_string = indoc! {"
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263"};
        println!("{}", example_string);
        let count = count_increases(example_string);
        assert_eq!(7, count);
    }
}
