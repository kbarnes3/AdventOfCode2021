#[allow(unused_imports)]
use day06_lanternfish_common::{SAMPLE_DATA, REAL_DATA};

const DAYS: u32 = 80;
const NEW_FISH_COUNTER: u32 = 8;
const RESET_COUNTER: u32 = 6;

fn main() {
    let result = do_work(&REAL_DATA);
    println!("{}", result);
}

fn do_work<const T: usize>(data: &[u32; T]) -> usize {
    let mut fish = data.to_vec();

    for _ in 0..DAYS {
        let mut fish_to_add = 0;

        for i in 0..fish.len() {
            if fish[i] == 0 {
                fish_to_add += 1;
                fish[i] = RESET_COUNTER;
            } else {
                fish[i] -= 1;
            }
        }

        for _ in 0..fish_to_add {
            fish.push(NEW_FISH_COUNTER);
        }
    }

    fish.len()
}
