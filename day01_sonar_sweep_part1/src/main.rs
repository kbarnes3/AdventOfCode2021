#[allow(unused_imports)]
use day01_sonar_sweep_common::{SAMPLE_DATA};

fn main() {
    let result: u32 = do_work(SAMPLE_DATA);
    println!("{}", result);}

fn do_work(data: [u32; 10]) -> u32 {
    let mut increases = 0;
    for i in 1..10 {
        if data[i] > data[i-1] {
            increases += 1;
        }
    }

    increases
}