#[allow(unused_imports)]
use day03_binary_diagnostic_common::{SAMPLE_DATA};

fn main() {
    let result: u32 = do_work(SAMPLE_DATA);
    println!("{}", result);
}

fn do_work<const N: usize>(data: [&str; N]) -> u32 {
    let number_length = data[0].chars().count();
    let mut gamma_string = String::new();
    let mut epsilon_string = String::new();

    for i in 0..number_length {
        let mut zero_count = 0;
        let mut one_count = 0;

        for number in data {
            let char = number.chars().nth(i);
            match char {
                Some('0') => zero_count += 1,
                Some('1') => one_count += 1,
                _ => ()
            }
        }

        if one_count > zero_count {
            gamma_string.push('1');
            epsilon_string.push('0');
        } else {
            gamma_string.push('0');
            epsilon_string.push('1');
        }
    }

    let gamma: u32 = u32::from_str_radix(&gamma_string, 2).unwrap();
    let epsilon: u32 = u32::from_str_radix(&epsilon_string, 2).unwrap();

    gamma * epsilon
}
