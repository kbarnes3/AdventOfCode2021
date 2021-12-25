use std::collections::{HashMap, VecDeque};

#[allow(unused_imports)]
use day10_syntax_scoring_common::{SAMPLE_DATA, REAL_DATA};

fn main() {
    let result = do_work(&REAL_DATA);
    println!("{}", result);
}

fn do_work<const T: usize>(data: &[&str; T]) -> u64 {
    let mut scores: Vec<u64> = Vec::new();
    let (matching_brackets, autocomplete_scores) = get_lookup_maps();

    for line in data {
        match evaluate_line(line, &matching_brackets, &autocomplete_scores) {
            LineType::Complete |
            LineType::Corrupted => (),
            LineType::Incomplete(line_score) => scores.push(line_score),
        }
    }

    scores.sort();
    let middle: usize = scores.len() / 2;

    scores[middle]
}

fn get_lookup_maps() -> (HashMap<char, char>, HashMap<char, u64>) {
    let mut matching_brackets: HashMap<char, char> = HashMap::with_capacity(4);
    matching_brackets.insert('(', ')');
    matching_brackets.insert('[', ']');
    matching_brackets.insert('{', '}');
    matching_brackets.insert('<', '>');

    let mut autocomplete_scores: HashMap<char, u64> = HashMap::with_capacity(4);
    autocomplete_scores.insert(')', 1);
    autocomplete_scores.insert(']', 2);
    autocomplete_scores.insert('}', 3);
    autocomplete_scores.insert('>', 4);

    (matching_brackets, autocomplete_scores)
}

enum LineType {
    Complete,
    Incomplete(u64),
    Corrupted,
}

fn evaluate_line(line: &str, matching_brackets: &HashMap<char, char>, autocomplete_scores: &HashMap<char, u64>) -> LineType {
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
                            return LineType::Corrupted;
                        }
                    }
                    None => {
                        // There are no more ending characters expected, so this is corrupt
                        // Check the score for this character
                        return LineType::Corrupted;
                    }
                }
            }
        }
    }

    if expected_endings.len() == 0 {
        // We are out of characters and we matched all the opening and closing brackets, so this is complete
        return LineType::Complete;
    } else {
        // We are out of characters but we have unmatched opening brackets, so this is incomplete. Find the
        // autocomplete score
        let mut score: u64 = 0;
        while !expected_endings.is_empty() {
            let ending: char = expected_endings.pop_back().unwrap();
            match autocomplete_scores.get(&ending) {
                Some(char_score) => {
                    score *= 5;
                    score += char_score;
                }
                None => panic!("Unscored ending character {}", ending)
            }
        }
        return LineType::Incomplete(score);
    }
}
