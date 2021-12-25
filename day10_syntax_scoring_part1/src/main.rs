use std::collections::{HashMap, VecDeque};

#[allow(unused_imports)]
use day10_syntax_scoring_common::{SAMPLE_DATA, REAL_DATA};

fn main() {
    let result = do_work(&REAL_DATA);
    println!("{}", result);
}

fn do_work<const T: usize>(data: &[&str; T]) -> u32 {
    let mut score: u32 = 0;
    let (matching_brackets, scores) = get_lookup_maps();

    for line in data {
        match evaluate_line(line, &matching_brackets, &scores) {
            LineType::Complete |
            LineType::Incomplete => (),
            LineType::Corrupted(line_score) => score += line_score
        }
    }

    score
}

fn get_lookup_maps() -> (HashMap<char, char>, HashMap<char, u32>) {
    let mut matching_brackets: HashMap<char, char> = HashMap::with_capacity(4);
    matching_brackets.insert('(', ')');
    matching_brackets.insert('[', ']');
    matching_brackets.insert('{', '}');
    matching_brackets.insert('<', '>');

    let mut scores: HashMap<char, u32> = HashMap::with_capacity(4);
    scores.insert(')', 3);
    scores.insert(']', 57);
    scores.insert('}', 1197);
    scores.insert('>', 25137);

    (matching_brackets, scores)
}

enum LineType {
    Complete,
    Incomplete,
    Corrupted(u32),
}

fn evaluate_line(line: &str, matching_brackets: &HashMap<char, char>, scores: &HashMap<char, u32>) -> LineType {
    let mut expected_endings: VecDeque<char> = VecDeque::new();
    for c in line.chars() {
        match matching_brackets.get(&c) {
            Some(ending) => {
                // c was a valid starting character, push the matching
                // ending character to the queue and continue
                expected_endings.push_back(*ending);
            }
            None => {
                // c should be an ending character. See if it is expected
                match expected_endings.back() {
                    Some(expected) => {
                        if c == *expected {
                            // c matches the next expected ending character,
                            // pop it from the queue and continue
                            expected_endings.pop_back();
                        } else {
                            // c didn't match the next ending character, so this line is corrupted
                            // Check the score for this character
                            return get_corrupt_score_for_char(c, scores);
                        }
                    }
                    None => {
                        // There are no more ending characters expected, so this is corrupt
                        // Check the score for this character
                        return get_corrupt_score_for_char(c, scores)
                    }
                }
            }
        }
    }

    if expected_endings.len() == 0 {
        // We are out of characters and we matched all the opening and closing brackets, so this is complete
        return LineType::Complete;
    } else {
        // We are out of characters but we have unmatched opening brackets, so this is incomplete
        return LineType::Incomplete;
    }
}

fn get_corrupt_score_for_char(c: char, scores: &HashMap<char, u32>) -> LineType {
    match scores.get(&c) {
        Some(score) => return LineType::Corrupted(*score),
        None => panic!("Unexpected character {}", c)
    }
}
