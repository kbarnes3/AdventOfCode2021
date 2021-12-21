#[allow(unused_imports)]
use day06_lanternfish_common::{SAMPLE_DATA, REAL_DATA};

const DAYS: u32 = 256;
const NEW_FISH_COUNTER: usize = 8;
const RESET_COUNTER: usize = 6;

fn main() {
    let result = do_work(&REAL_DATA);
    println!("{}", result);
}

fn do_work<const T: usize>(data: &[usize; T]) -> usize {
    let mut fish: [usize; NEW_FISH_COUNTER + 1] = [0; NEW_FISH_COUNTER + 1];
    populate_initial_array(data, &mut fish);

    for _ in 0..DAYS {
        let fish_to_add = fish[0];

        for i in 1..=NEW_FISH_COUNTER {
            fish[i - 1] = fish[i];
        }

        fish[RESET_COUNTER] += fish_to_add;
        fish[NEW_FISH_COUNTER] = fish_to_add;
    }

    get_total_fish(&fish)
}

fn populate_initial_array<const T: usize>(data: &[usize; T], fish: &mut [usize; NEW_FISH_COUNTER + 1]) {
    for i in data {
        fish[*i] += 1;
    }
}

fn get_total_fish(fish: &[usize; NEW_FISH_COUNTER + 1]) -> usize {
    let mut total_fish = 0;
    for i in fish {
        total_fish += *i;
    }

    total_fish
}