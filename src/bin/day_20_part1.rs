use std::{collections::HashMap, fs};

use itertools::Itertools;

fn main() {
    let filename = "day21_input.txt";
    let contents = fs::read_to_string(filename).expect("File read error");

    println!("{}", contents);

    let result = do_the_thing(&contents);

    println!("Result {}", result);
}

fn do_the_thing(input: &str) -> u128 {
    let mut lines = input.lines();

    let algorithm = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            '#' => 1,
            '.' => 0,
            _ => panic!("invalid character {}", c),
        })
        .collect_vec(); // First line is the algo.
    lines.next();
    // TODO: Reimplement with HashMap (i, j) bool
    let mut image = lines
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().map(move |(j, c)| {
                let pixel = match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("invalid character {}", c),
                };
                let key = (i, j);
                (key, pixel)
            }) // Gives an iterator of (key, value) pairs
        })
        .collect::<HashMap<_, _>>();

    for _ in 0..10 {
        image = enhance_image(image, &algorithm);
    }
    image
        .values()
        .filter(|&&pixel| pixel)
        .count()
        .try_into()
        .unwrap()
}

fn enhance_image(
    image: HashMap<(usize, usize), bool>,
    algorithm: &[i32],
) -> HashMap<(usize, usize), bool> {
    let new_image = HashMap::new();

    new_image
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let input = indoc! {"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
        
        #..#.
        #....
        ##..#
        ..#..
        ..###"};

        let result = do_the_thing(input);

        assert_eq!(35, result);
    }
}
