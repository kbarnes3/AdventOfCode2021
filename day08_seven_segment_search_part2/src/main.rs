use std::collections::HashSet;

#[allow(unused_imports)]
use day08_seven_segment_search_common::{Display, SAMPLE_DATA, REAL_DATA};

fn main() {
    let result = do_work(&REAL_DATA);
    println!("{}", result);
}

struct Mapping {
    pub top: char,
    pub up_left: char,
    pub up_right: char,
    pub center: char,
    pub down_left: char,
    pub down_right: char,
    pub bottom: char
}

impl Mapping {
    fn new() -> Mapping {
        Mapping { top: '\0', up_left: '\0', up_right: '\0', center: '\0', down_left: '\0', down_right: '\0', bottom: '\0' }
    }

    fn ensure_valid(&self) {
        let mut seen_chars: HashSet<char> = HashSet::with_capacity(7);
        // top
        if self.top == '\0' {
            panic!("top not known");
        }
        if (self.top < 'a') || (self.top > 'g') {
            panic!("Unexpected value: {}", self.top);
        }
        if !seen_chars.insert(self.top) {
            panic!("{} already used", self.top);
        }

        // up_left
        if self.up_left == '\0' {
            panic!("up_left not known");
        }
        if (self.up_left < 'a') || (self.up_left > 'g') {
            panic!("Unexpected value: {}", self.up_left);
        }
        if !seen_chars.insert(self.up_left) {
            panic!("{} already used", self.up_left);
        }

        // up_right
        if self.up_right == '\0' {
            panic!("up_right not known");
        }
        if (self.up_right < 'a') || (self.up_right > 'g') {
            panic!("Unexpected value: {}", self.up_right);
        }
        if !seen_chars.insert(self.up_right) {
            panic!("{} already used", self.up_right);
        }

        // center
        if self.center == '\0' {
            panic!("center not known");
        }
        if (self.center < 'a') || (self.center > 'g') {
            panic!("Unexpected value: {}", self.center);
        }
        if !seen_chars.insert(self.center) {
            panic!("{} already used", self.center);
        }

        // down_left
        if self.down_left == '\0' {
            panic!("down_left not known");
        }
        if (self.down_left < 'a') || (self.down_left > 'g') {
            panic!("Unexpected value: {}", self.down_left);
        }
        if !seen_chars.insert(self.down_left) {
            panic!("{} already used", self.down_left);
        }

        // down_right
        if self.down_right == '\0' {
            panic!("down_right not known");
        }
        if (self.down_right < 'a') || (self.down_right > 'g') {
            panic!("Unexpected value: {}", self.down_right);
        }
        if !seen_chars.insert(self.down_right) {
            panic!("{} already used", self.down_right);
        }

        // bottom
        if self.bottom == '\0' {
            panic!("bottom not known");
        }
        if (self.bottom < 'a') || (self.bottom > 'g') {
            panic!("Unexpected value: {}", self.bottom);
        }
        if !seen_chars.insert(self.bottom) {
            panic!("{} already used", self.bottom);
        }
    }
}

fn do_work<const T: usize>(data: &[Display; T]) -> u32 {
    let mut output_sum: u32 = 0;

    for display in data {
        let mapping: Mapping = compute_mapping(&display.patterns);
        let mut output: u32;
        output = 1000 * get_digit(&mapping, display.output[0]);
        output += 100 * get_digit(&mapping, display.output[1]);
        output += 10 * get_digit(&mapping, display.output[2]);
        output += get_digit(&mapping, display.output[3]);
        output_sum += output;
    }

    output_sum
}

fn compute_mapping(all_patterns: &[&[char]; 10]) -> Mapping {
    let mut mapping = Mapping::new();

    let mut one: Option<&[char]> = None;
    let mut four: Option<&[char]> = None;
    let mut seven: Option<&[char]> = None;

    for pattern in all_patterns {
        match pattern.len() {
            2 => one = Some(pattern),
            3 => seven = Some(pattern),
            4 => four = Some(pattern),
            _ => ()
        }
    }

    let one = one.unwrap();
    let four = four.unwrap();
    let seven = seven.unwrap();

    // The top segment is the one in 7 not in 1
    for c in seven {
        if !one.contains(&c) {
            mapping.top = *c;
        }
    }
    
    // 4 and 7 have all the same segments as 9 except the bottom segment
    let mut almost_nine: HashSet<char> = HashSet::with_capacity(5);

    for c in four {
        almost_nine.insert(*c);
    }
    for c in seven {
        almost_nine.insert(*c);
    }

    let (bottom_segment, nine) = find_added_segment(&almost_nine, all_patterns);
    mapping.bottom = bottom_segment;

    // The segment not found in 9 is the down left segment
    for c in 'a'..='g' {
        if !nine.contains(&c) {
            mapping.down_left = c;
        }
    }

    // 0 is the segments in 1, the top, bottom, and down_left, plus the up_left
    let mut almost_zero: HashSet<char> = HashSet::with_capacity(5);
    for c in one {
        almost_zero.insert(*c);
    }
    almost_zero.insert(mapping.top);
    almost_zero.insert(mapping.bottom);
    almost_zero.insert(mapping.down_left);

    let (up_left_segment, zero) = find_added_segment(&almost_zero, all_patterns);
    mapping.up_left = up_left_segment;

    // The center segment is not found in 0
    for c in 'a'..='g' {
        if !zero.contains(&c) {
            mapping.center = c;
        }
    }

    // 5 is the known top, up_left, center, and bottom with the unknown down_right
    let mut almost_five: HashSet<char> = HashSet::with_capacity(4);
    almost_five.insert(mapping.top);
    almost_five.insert(mapping.up_left);
    almost_five.insert(mapping.center);
    almost_five.insert(mapping.bottom);

    let (down_right_segment, _/*five*/) = find_added_segment(&almost_five, all_patterns);
    mapping.down_right = down_right_segment;

    // The up_right is the in 1 and not the bottom_right
    if one[0] == mapping.down_right {
        mapping.up_right = one[1];
    } else {
        mapping.up_right = one[0];
    }

    mapping.ensure_valid();
    mapping
}

fn find_added_segment<'a>(other_segments: &HashSet<char>, all_patterns: &[&'a[char]; 10]) -> (char, &'a[char]) {
    for pattern in all_patterns {
        if pattern.len() != (other_segments.len() + 1) {
            continue;
        }

        let mut unseen_segment: char = '\0';
        let mut unseen_count: usize = 0;
        for c in *pattern {
            if !other_segments.contains(c) {
                unseen_segment = *c;
                unseen_count += 1;
            }
        }

        if unseen_count == 1 {
            let new_segment: char = unseen_segment;
            let new_number: &[char] = pattern;
            return (new_segment, new_number);
        }
    }

    panic!("Couldn't find the number with added segment")
}

fn get_digit(mapping: &Mapping, segments: &[char]) -> u32 {
    if segments.contains(&mapping.top) {
        // 0, 2, 3, 5, 6, 7, 8, 9
        if segments.contains(&mapping.up_left) {
            // 0, 5, 6, 8, 9
            if segments.contains(&mapping.up_right) {
                // 0, 8, 9
                if segments.contains(&mapping.center) {
                    // 8, 9
                    if segments.contains(&mapping.down_left) {
                        // 8
                        return 8;
                    } else {
                        // 9
                        return 9;
                    }
                } else {
                    // 0
                    return 0;
                }
            } else {
                // 5, 6
                if segments.contains(&mapping.down_left) {
                    // 6
                    return 6;
                } else {
                    // 5
                    return 5;
                }
            }
        } else {
            // 2, 3, 7
            if segments.contains(&mapping.center) {
                // 2, 3
                if segments.contains(&mapping.down_left) {
                    // 2
                    return 2;
                } else {
                    // 3
                    return 3;
                }
            } else {
                // 7
                return 7;
            }
        }
    } else {
        // 1, 4
        if segments.contains(&mapping.up_left) {
            // 4
            return 4;
        } else {
            // 1
            return 1;
        }
    }
}