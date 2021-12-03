pub enum Direction {
    Forward,
    Down,
    Up
}

pub struct Command {
    pub direction: Direction,
    pub distance: u32,
}

use Direction::*;

// Substitute with
// '<,'>s/^\(\w*\) \(\d*\)/    Command { direction: \1, distance: \2 },/ | '<,'>s/forward/Forward/ | '<,'>s/down/Down/ | '<,'>s/up/Up/

pub const SAMPLE_DATA: [Command; 6] = [
    Command { direction: Forward, distance: 5 },
    Command { direction: Down, distance: 5 },
    Command { direction: Forward, distance: 8 },
    Command { direction: Up, distance: 3 },
    Command { direction: Down, distance: 8 },
    Command { direction: Forward, distance: 2 },
];
