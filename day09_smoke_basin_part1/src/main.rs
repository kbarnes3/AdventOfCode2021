#[allow(unused_imports)]
use day09_smoke_basin_common::{SAMPLE_DATA, REAL_DATA};

fn main() {
    let result = do_work(&REAL_DATA);
    println!("{}", result);
}

fn do_work<const X: usize, const Y: usize>(data: &[[u32; X]; Y]) -> u32 {
    let mut risk_levels: u32 = 0;

    for y in 0..Y {
        for x in 0..X {
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

            if local_min {
                risk_levels += current_height + 1;
            }
        }
    }

    risk_levels
}
