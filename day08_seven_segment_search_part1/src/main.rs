use std::collections::HashSet;

#[allow(unused_imports)]
use day08_seven_segment_search_common::{Display, SAMPLE_DATA};

const EASY_SEGMENT_COUNTS: [usize; 4] = [2, 3, 4, 7];

fn main() {
    let result = do_work(&SAMPLE_DATA);
    println!("{}", result);
}

fn do_work<const T: usize>(data: &[Display; T]) -> usize {
    let easy_segments: HashSet<usize> = get_easy_segment_hashset();
    let mut easy_count: usize = 0;

    for display in data {
        for digit in display.output {
            if easy_segments.contains(&digit.len()) {
                easy_count += 1;
            }
        }
    }

    easy_count
}

fn get_easy_segment_hashset() -> HashSet<usize> {
    let mut set: HashSet<usize> = HashSet::with_capacity(EASY_SEGMENT_COUNTS.len());

    for count in EASY_SEGMENT_COUNTS {
        set.insert(count);
    }

    set
}
