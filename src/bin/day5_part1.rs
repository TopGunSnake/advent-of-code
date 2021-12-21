use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn main() {
    let filename = "day5_input.txt";
    let contents = fs::read_to_string(filename).expect("File read error");

    println!("{}", contents);

    let result = find_overlaps(&contents);

    println!("Result: {}", result);
}

fn find_overlaps(input: &str) -> u32 {
    let mut grid = Grid::new();
    for slice in input.lines() {
        let (left, right) = slice.split_terminator(" -> ").collect_tuple().unwrap();
        let (left, right) = (Point::parse(left), Point::parse(right));

        let seg = Segment::between(left, right);

        grid.add_segment(seg);
        // println!("Grid for {}: {}", slice, grid);
    }
    // println!("Final Grid: {}", grid);
    grid.overlaps()
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn parse(input: &str) -> Self {
        let (x, y) = input
            .split_terminator(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();
        Point { x, y }
    }
}

struct Segment {
    a: Point,
    b: Point,
    orientation: Orientation,
}

enum Orientation {
    Horizontal,
    Vertical,
    Diagonal,
}

impl Segment {
    fn between(left: Point, right: Point) -> Self {
        let orientation = if left.x == right.x {
            Orientation::Vertical
        } else if left.y == right.y {
            Orientation::Horizontal
        } else {
            Orientation::Diagonal
        };

        Segment {
            a: left,
            b: right,
            orientation,
        }
    }

    fn get_points(&self) -> Vec<Point> {
        match self.orientation {
            Orientation::Horizontal => {
                let y = self.a.y;
                let xs = match self.a.x <= self.b.x {
                    true => self.a.x..=self.b.x,
                    false => self.b.x..=self.a.x,
                };
                xs.map(|x| Point { x, y }).collect_vec()
            }
            Orientation::Vertical => {
                let x = self.a.x;
                let ys = match self.a.y <= self.b.y {
                    true => self.a.y..=self.b.y,
                    false => self.b.y..=self.a.y,
                };
                ys.map(|y| Point { x, y }).collect_vec()
            }
            Orientation::Diagonal => {
                Vec::new()
            }
        }
    }
}

#[derive(Debug)]
struct Grid {
    grid: HashMap<Point, u32>,
    width: u32,
    height: u32,
}

impl Grid {
    fn new() -> Self {
        Grid {
            grid: HashMap::new(),
            width: 0,
            height: 0,
        }
    }

    fn add_segment(&mut self, seg: Segment) {
        let points = seg.get_points();
        for point in points {
            // println!("{:?}", point);
            if let Some(value) = self.grid.get_mut(&point) {
                *value += 1;
            } else {
                self.width = self.width.max(point.x + 1);
                self.height = self.height.max(point.y + 1);

                self.grid.insert(point, 1);
            }
        }
    }

    fn overlaps(&self) -> u32 {
        let mut count = 0;
        for (_key, &value) in self.grid.iter() {
            if value >= 2 {
                count += 1;
            }
        }
        count
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = std::iter::repeat(
            std::iter::repeat(0)
                .take(self.width as usize)
                .collect::<Vec<u32>>(),
        )
        .take(self.height as usize)
        .collect::<Vec<Vec<u32>>>();
        for (point, &value) in self.grid.iter() {
            // println!("x: {}, y: {}, output: {:?}", point.x, point.y, output);
            output[point.y as usize][point.x as usize] = value;
        }
        write!(f, "\n[\n")?;
        for row in output {
            writeln!(f, " {:?}", row)?;
        }
        write!(f, "]")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2";

        let result = find_overlaps(input);

        assert_eq!(5, result);
    }
}
