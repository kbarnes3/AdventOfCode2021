#[allow(unused_imports)]
use day10_syntax_scoring_common::{SAMPLE_DATA};

fn main() {
    let result = do_work(&SAMPLE_DATA);
    println!("{}", result);
}

fn do_work(data: &u32) -> u32 {
    *data
}
