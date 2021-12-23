use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use day09_smoke_basin_common::{SAMPLE_DATA, REAL_DATA};

fn main() {
    let result = do_work(&REAL_DATA);
    println!("{}", result);
}

fn do_work<const X: usize, const Y: usize>(data: &[[u32; X]; Y]) -> usize {
    let mut basin_sizes: Vec<usize> = Vec::new();

    for y in 0..Y {
        for x in 0..X {
            if is_local_min(data, x, y) {
                let basin_size = get_basin_size(data, x, y);
                basin_sizes.push(basin_size);
            }
        }
    }

    basin_sizes.sort();
    basin_sizes.reverse();

    basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
}

fn is_local_min<const X: usize, const Y: usize>(data: &[[u32; X]; Y], x: usize, y: usize) -> bool {
    let current_height = data[y][x];
    let mut local_min = true;

    if (x > 0) && (data[y][x-1] <= current_height) {
        local_min = false;
    }

    if (x < X - 1) && (data[y][x+1] <= current_height) {
        local_min = false;
    }

    if (y > 0) && (data[y-1][x] <= current_height) {
        local_min = false;
    }
    if (y < Y - 1) && (data[y+1][x] <= current_height) {
        local_min = false;
    }

    local_min
}

fn get_basin_size<const X: usize, const Y: usize>(data: &[[u32; X]; Y], min_x: usize, min_y: usize) -> usize {
    let mut basin_size = 0;
    let mut points_to_check: VecDeque<(usize, usize)> = VecDeque::new();
    let mut seen_points: HashSet<(usize, usize)> = HashSet::new();

    points_to_check.push_back((min_x, min_y));
    seen_points.insert((min_x, min_y));

    while !points_to_check.is_empty() {
        let (x, y) = points_to_check.pop_front().unwrap();
        basin_size += 1;

        if x > 0 {
            queue_point_if_in_basin(data, x - 1, y, &mut points_to_check, &mut seen_points);
        }

        if x < X - 1 {
            queue_point_if_in_basin(data, x + 1, y, &mut points_to_check, &mut seen_points);
        }

        if y > 0 {
            queue_point_if_in_basin(data, x, y - 1, &mut points_to_check, &mut seen_points);
        }

        if y < Y - 1 {
            queue_point_if_in_basin(data, x, y + 1, &mut points_to_check, &mut seen_points);
        }
    }

    basin_size
}

fn queue_point_if_in_basin<const X: usize, const Y: usize>(
    data: &[[u32; X]; Y],
    x: usize,
    y: usize,
    points_to_check: &mut VecDeque<(usize, usize)>,
    seen_points: &mut HashSet<(usize, usize)>
) {
    const STOP_HEIGHT: u32 = 9;

    if data[y][x] != STOP_HEIGHT {
        if seen_points.insert((x, y)) {
            points_to_check.push_back((x, y));
        }
    }

}
