#[allow(unused_imports)]
use day07_the_treachery_of_whales_common::{SAMPLE_DATA, REAL_DATA};

fn main() {
    let result = do_work(&REAL_DATA);
    println!("{}", result);
}

fn do_work<const T: usize>(data: &[u32; T]) -> u32 {
    let min = *(data.iter().min().unwrap());
    let max = *(data.iter().max().unwrap());

    let mut best_cost = get_fuel_cost(data, min);
    for i in min+1..=max {
        let cost = get_fuel_cost(data, i);
        if cost < best_cost {
            best_cost = cost;
        }
    }

    best_cost
}

fn get_fuel_cost<const T: usize>(data: &[u32; T], target_pos: u32) -> u32 {
    let mut cost: u32 = 0;

    for current_pos in data {
        if *current_pos < target_pos {
            cost += target_pos - *current_pos;
        } else {
            cost += *current_pos - target_pos;
        }
    }

    cost
}
