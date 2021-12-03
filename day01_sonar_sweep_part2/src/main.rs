#[allow(unused_imports)]
use day01_sonar_sweep_common::{SAMPLE_DATA, REAL_DATA};

fn main() {
    let result: u32 = do_work(REAL_DATA);
    println!("{}", result);}

fn do_work<const N: usize>(data: [u32; N]) -> u32 {
    if N < 3 {
        panic!("data must have at least 3 values");
    }

    let mut increases = 0;
    let mut last_sum = data[0] + data[1] + data[2];

    for i in 1..N-2 {
        let sum = data[i] + data[i+1] + data[i+2];
        if sum > last_sum {
            increases += 1;
        }

        last_sum = sum;
    }

    increases
}
