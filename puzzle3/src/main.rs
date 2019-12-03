extern crate itertools;

use itertools::Itertools;
use std::fmt;
use std::io;

type Vector2D = (u32, u32);

#[derive(PartialEq)]
enum Orientation {
    Vertical,
    Horizontal,
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Orientation::Vertical => write!(f, "Vertical"),
            Orientation::Horizontal => write!(f, "Horizontal"),
        }
    }
}

struct Line {
    start: Vector2D,
    end: Vector2D,
    orientation: Orientation,
}

impl Line {
    fn new(start: Vector2D, end: Vector2D) -> Line {
        Line {
            start,
            end,
            orientation: if start.0 == end.0 {
                Orientation::Vertical
            } else {
                Orientation::Horizontal
            },
        }
    }

    fn intersect(&self, other: &Line) -> Option<Vector2D> {
        // Parallel lines don't intersect
        if self.orientation == other.orientation {
            return None;
        }

        let (horizontal_line, vertical_line) = match self.orientation {
            Orientation::Horizontal => (self, other),
            Orientation::Vertical => (other, self),
        };

        // Check if lines intersect horizontally
        if vertical_line.start.0 < horizontal_line.start.0
            || vertical_line.start.0 > horizontal_line.end.0
        {
            return None;
        }

        // Check if lines intersect vertically
        if horizontal_line.start.1 < vertical_line.start.1
            || horizontal_line.start.1 > vertical_line.end.1
        {
            return None;
        }

        Some((vertical_line.start.0, horizontal_line.start.1))
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Line {}x{} -> {}x{} ({})",
            self.start.0, self.start.1, self.end.0, self.end.1, self.orientation
        )
    }
}

fn parse_path(path_str: &str, central_port: Vector2D) -> Vec<Line> {
    let mut last_point = central_port;

    path_str
        .split(',')
        .map(|step| {
            let value: u32 = match step[1..].parse() {
                Ok(v) => v,
                Err(_) => panic!("Error parsing direction value {}!", &step[1..]),
            };
            let new_point = match step.chars().next().unwrap() {
                'R' => (last_point.0 + value, last_point.1),
                'D' => (last_point.0, last_point.1 - value),
                'L' => (last_point.0 - value, last_point.1),
                'U' => (last_point.0, last_point.1 + value),
                _dir => panic!("Invalid direction {}", _dir),
            };
            let l = Line::new(last_point, new_point);
            last_point = new_point;
            l
        })
        .collect()
}

fn manhatten_distance(point1: Vector2D, point2: Vector2D) -> i32 {
    (point1.0 as i32 - point2.0 as i32).abs() + (point1.1 as i32 - point2.1 as i32).abs()
}

fn main() -> io::Result<()> {
    // Not very elegant, but this keeps all coordinates positive
    let central_port = (10000, 10000);

    let lines: Vec<Vec<Line>> = include_str!("../input")
        .trim()
        .split('\n')
        .map(|x| parse_path(x, central_port))
        .collect();

    let min_distance: i32 = lines[0]
        .iter()
        .cartesian_product(lines[1].iter())
        .filter_map(|(line1, line2)| line1.intersect(&line2))
        .map(|x| manhatten_distance(x, central_port))
        .min()
        .unwrap();

    println!("Minimum: {}", min_distance);

    Ok(())
}
