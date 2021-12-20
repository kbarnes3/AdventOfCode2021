use std::collections::HashMap;

#[allow(unused_imports)]
use day05_hydrothermal_venture_common::{Point, SAMPLE_DATA};

fn main() {
    let result = do_work(&SAMPLE_DATA);
    println!("{}", result);
}

enum LineDirection {
    Horizontal,
    Vertical,
    Diagonal
}

fn do_work<const T: usize>(data: &[(Point, Point); T]) -> u32 {
    let mut covered_spaces: HashMap<Point, usize> = HashMap::new();

    for (point1, point2) in data {
        match get_line_direction(point1, point2) {
            LineDirection::Horizontal => mark_horizontal_line(point1, point2, &mut covered_spaces),
            LineDirection::Vertical => mark_vertical_line(point1, point2, &mut covered_spaces),
            LineDirection::Diagonal => (),
        }
    }

    let mut double_covered = 0;
    for (_, value) in covered_spaces {
        if value >= 2 {
            double_covered += 1;
        }
    }
    double_covered
}

fn get_line_direction(point1: &Point, point2: &Point) -> LineDirection {
    if point1.x == point2.x {
        return LineDirection::Horizontal;
    }
    if point1.y == point2.y {
        return LineDirection::Vertical;
    }

    LineDirection::Diagonal
}

fn mark_horizontal_line(point1: &Point, point2: &Point, covered_spaces: &mut HashMap<Point, usize>) {
    if point1.x != point2.x {
        panic!("Not a horizontal line!");
    }

    let start_y: u32;
    let end_y: u32;
    if point1.y < point2.y {
        start_y = point1.y;
        end_y = point2.y;
    } else {
        start_y = point2.y;
        end_y = point1.y;
    }
    for y in start_y..=end_y {
        increment_covered_space(point1.x, y, covered_spaces);
    }
}

fn mark_vertical_line(point1: &Point, point2: &Point, covered_spaces: &mut HashMap<Point, usize>) {
    if point1.y != point2.y {
        panic!("Not a vertical line!");
    }

    let start_x: u32;
    let end_x: u32;
    if point1.x < point2.x {
        start_x = point1.x;
        end_x = point2.x;
    } else {
        start_x = point2.x;
        end_x = point1.x;
    }
    for x in start_x..=end_x {
        increment_covered_space(x, point1.y, covered_spaces);
    }
}

fn increment_covered_space(x: u32, y: u32, covered_spaces: &mut HashMap<Point, usize>) {
    let point = Point {x: x, y: y};
    let new_count;
    match covered_spaces.get(&point) {
        Some(count) => new_count = count + 1,
        None => new_count = 1
    };

    covered_spaces.insert(point, new_count);
}