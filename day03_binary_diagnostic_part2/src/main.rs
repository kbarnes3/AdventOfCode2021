#[allow(unused_imports)]
use day03_binary_diagnostic_common::{SAMPLE_DATA, REAL_DATA};

fn main() {
    let result: u32 = do_work(REAL_DATA);
    println!("{}", result);
}

fn do_work<const N: usize>(data: [&str; N]) -> u32 {
    let number_length = data[0].chars().count();

    // Oxygen
    let mut oxygen_vec = Vec::from(data);
    let mut oxygen = 0;

    for i in 0..number_length {
        let mut zero_vec = Vec::new();
        let mut one_vec = Vec::new();

        for number in oxygen_vec {
            let char = number.chars().nth(i);
            match char {
                Some('0') => zero_vec.push(number),
                Some('1') => one_vec.push(number),
                _ => ()
            }
        }

        if one_vec.len() >= zero_vec.len() {
            oxygen_vec = one_vec;
        } else {
            oxygen_vec = zero_vec;
        }

        if oxygen_vec.len() == 1 {
            oxygen = u32::from_str_radix(oxygen_vec[0], 2).unwrap();
            break;
        }
    }

    // CO2
    let mut co2_vec = Vec::from(data);
    let mut co2 = 0;

    for i in 0..number_length {
        let mut zero_vec = Vec::new();
        let mut one_vec = Vec::new();

        for number in co2_vec {
            let char = number.chars().nth(i);
            match char {
                Some('0') => zero_vec.push(number),
                Some('1') => one_vec.push(number),
                _ => ()
            }
        }

        if zero_vec.len() <= one_vec.len() {
            co2_vec = zero_vec;
        } else {
            co2_vec = one_vec;
        }

        if co2_vec.len() == 1 {
            co2 = u32::from_str_radix(co2_vec[0], 2).unwrap();
            break;
        }
    }

    oxygen * co2
}
