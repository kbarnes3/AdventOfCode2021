use std::collections::HashMap;

#[allow(unused_imports)]
use day07_the_treachery_of_whales_common::{SAMPLE_DATA, REAL_DATA};

fn main() {
    let result = do_work(&REAL_DATA);
    println!("{}", result);
}

fn do_work<const T: usize>(data: &[u32; T]) -> u32 {
    let mut cached_moves: HashMap<u32, u32> = HashMap::new();
    let min = *(data.iter().min().unwrap());
    let max = *(data.iter().max().unwrap());

    let mut best_cost = get_fuel_cost(data, min, &mut cached_moves);
    for i in min+1..=max {
        let cost = get_fuel_cost(data, i, &mut cached_moves);
        if cost < best_cost {
            best_cost = cost;
        }
    }

    best_cost
}

fn get_fuel_cost<const T: usize>(data: &[u32; T], target_pos: u32, cached_moves: &mut HashMap<u32, u32>) -> u32 {
    let mut cost: u32 = 0;

    for current_pos in data {
        let distance: u32;

        if *current_pos < target_pos {
            distance = target_pos - *current_pos;
        } else {
            distance = *current_pos - target_pos;
        }

        cost += get_move_cost(distance, cached_moves)
    }

    cost
}

fn get_move_cost(distance: u32, cached_moves: &mut HashMap<u32, u32>) -> u32 {
    match cached_moves.get(&distance) {
        Some(cost) => return *cost,
        None => {
            let cost = compute_move_cost(distance);
            cached_moves.insert(distance, cost);
            return cost;
        }
    }
}

fn compute_move_cost(distance: u32) -> u32 {
    let mut cost: u32 = 0;

    for i in 1..=distance {
        cost += i;
    }

    cost
}
