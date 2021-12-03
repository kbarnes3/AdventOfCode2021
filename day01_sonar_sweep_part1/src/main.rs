#[allow(unused_imports)]
use day01_sonar_sweep_common::{SAMPLE_DATA, REAL_DATA};

fn main() {
    let result: u32 = do_work(REAL_DATA);
    println!("{}", result);}

fn do_work<const N: usize>(data: [u32; N]) -> u32 {
    if N < 1 {
        panic!("data must have at least 1 value");
    }

    let mut increases = 0;
    for i in 1..N {
        if data[i] > data[i-1] {
            increases += 1;
        }
    }

    increases
}