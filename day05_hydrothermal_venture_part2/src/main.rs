use std::collections::HashMap;

#[allow(unused_imports)]
use day05_hydrothermal_venture_common::{Point, SAMPLE_DATA, REAL_DATA};

fn main() {
    let result = do_work(&REAL_DATA);
    println!("{}", result);
}

fn do_work<const T: usize>(data: &[(Point, Point); T]) -> u32 {
    let mut covered_spaces: HashMap<Point, usize> = HashMap::new();

    for (point1, point2) in data {
        mark_line(point1, point2, &mut covered_spaces)
    }

    let mut double_covered = 0;
    for (_, value) in covered_spaces {
        if value >= 2 {
            double_covered += 1;
        }
    }
    double_covered
}

enum Direction {
    Forwards,
    Backwards,
    Zero
}

fn mark_line(point1: &Point, point2: &Point, covered_spaces: &mut HashMap<Point, usize>) {
    let x_direction: Direction;
    let y_direction: Direction;

    if point1.x < point2.x {
        x_direction = Direction::Forwards;
    } else if point1.x > point2.x {
        x_direction = Direction::Backwards;
    } else {
        x_direction = Direction::Zero;
    }

    if point1.y < point2.y {
        y_direction = Direction::Forwards;
    } else if point1.y > point2.y {
        y_direction = Direction::Backwards;
    } else {
        y_direction = Direction::Zero;
    }

    let mut x = point1.x;
    let mut y = point1.y;
    increment_covered_space(x, y, covered_spaces);

    while !((point2.x == x) && (point2.y == y)) {
        match x_direction {
            Direction::Forwards => x += 1,
            Direction::Backwards => x -= 1,
            Direction::Zero => (),
        }
        match y_direction {
            Direction::Forwards => y += 1,
            Direction::Backwards => y -= 1,
            Direction::Zero => ()
        }
        increment_covered_space(x, y, covered_spaces);
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