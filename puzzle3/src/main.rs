extern crate itertools;

use itertools::Itertools;
use std::fmt;
use std::io;

type Vector2D = (i32, i32);

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
    real_start: Vector2D,
    orientation: Orientation,
}

impl Line {
    fn new(start: Vector2D, end: Vector2D) -> Line {
        let orientation = if start.0 == end.0 {
            Orientation::Vertical
        } else {
            Orientation::Horizontal
        };

        let real_start = start;
        let (start, end) = match orientation {
            Orientation::Vertical => {
                if start.1 > end.1 {
                    (end, start)
                } else {
                    (start, end)
                }
            }
            Orientation::Horizontal => {
                if start.0 > end.0 {
                    (end, start)
                } else {
                    (start, end)
                }
            }
        };

        Line {
            start,
            end,
            real_start,
            orientation,
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

    fn contains_point(&self, point: Vector2D) -> bool {
        point.0 >= self.start.0
            && point.0 <= self.end.0
            && point.1 >= self.start.1
            && point.1 <= self.end.1
    }

    fn steps(&self) -> i32 {
        manhatten_distance(self.start, self.end)
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

fn parse_path(path_str: &str) -> Vec<Line> {
    let mut last_point = (0, 0);

    path_str
        .split(',')
        .map(|step| {
            let value: i32 = match step[1..].parse() {
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
    (point1.0 - point2.0).abs() + (point1.1 - point2.1).abs()
}

fn steps_to_point(lines: &Vec<Line>, point: Vector2D) -> i32 {
    let mut steps = 0;
    let mut num_lines = 0;
    for line in lines {
        if line.contains_point(point) {
            steps += manhatten_distance(line.real_start, point);
            break;
        }

        steps += line.steps();
        num_lines += 1;
    }

    steps
}

fn main() -> io::Result<()> {
    let lines: Vec<Vec<Line>> = include_str!("../input")
        .trim()
        .split('\n')
        .map(|x| parse_path(x))
        .collect();

    let intersections: Vec<Vector2D> = lines[0]
        .iter()
        .cartesian_product(lines[1].iter())
        .filter_map(|(line1, line2)| line1.intersect(&line2))
        .filter(|&x| x != (0, 0))
        .collect();

    println!("Intersections: {:?}", intersections);

    let min_distance: i32 = intersections
        .iter()
        .map(|&x| manhatten_distance(x, (0, 0)))
        .min()
        .unwrap();

    println!("Minimum: {}", min_distance);

    let minimum_steps: i32 = intersections
        .iter()
        .map(|&x| steps_to_point(&lines[0], x) + steps_to_point(&lines[1], x))
        .min()
        .unwrap();

    println!("Minimum steps: {}", minimum_steps);

    Ok(())
}
