use std::fs;

fn main() {
    let filename = "day9_input.txt";
    let contents = fs::read_to_string(filename).expect("File read error");

    println!("{}", contents);

    let result = do_the_thing(&contents);

    println!("Result {}", result);
}

fn do_the_thing(input: &str) -> u128 {
    let data = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut risk = 0;

    for (i, line) in data.iter().enumerate() {
        for (j, value) in line.iter().enumerate() {
            // Check the neighbors (up, down, left, then right)
            let accessor = |x: usize, y: usize| data.get(x).and_then(|line| line.get(y));

            let neighbors = [
                if i == 0 { None } else { accessor(i - 1, j) },
                accessor(i + 1, j),
                if j == 0 { None } else { accessor(i, j - 1) },
                accessor(i, j + 1),
            ];

            let value_is_local_min =
                neighbors
                    .iter()
                    // .inspect(|n| println!("{:#?}", n))
                    .all(|n| match n {
                        Some(&v) => v > *value,
                        None => true,
                    });

            if value_is_local_min {
                risk += (1 + value) as u128;
            }
        }
    }

    risk
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let input = indoc! {"
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
        "};

        let result = do_the_thing(input);

        assert_eq!(15, result);
    }
}
