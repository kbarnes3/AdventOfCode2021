#[allow(unused_imports)]
use day07_the_treachery_of_whales_common::{SAMPLE_DATA};

fn main() {
    let result = do_work(&SAMPLE_DATA);
    println!("{}", result);
}

fn do_work(data: &u32) -> u32 {
    *data
}