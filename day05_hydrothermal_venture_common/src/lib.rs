#[derive(PartialEq, Eq, Hash)]
pub struct Point {
    pub x: u32,
    pub y: u32
}

// Substitute with
// '<,'>s/\(\d\+\),\(\d\+\) -> \(\d\+\),\(\d\+\)/    (Point { x: \1, y: \2 }, Point { x: \3, y: \4 }),/
pub const SAMPLE_DATA: [(Point, Point); 10] = [
    (Point { x: 0, y: 9 }, Point { x: 5, y: 9 }),
    (Point { x: 8, y: 0 }, Point { x: 0, y: 8 }),
    (Point { x: 9, y: 4 }, Point { x: 3, y: 4 }),
    (Point { x: 2, y: 2 }, Point { x: 2, y: 1 }),
    (Point { x: 7, y: 0 }, Point { x: 7, y: 4 }),
    (Point { x: 6, y: 4 }, Point { x: 2, y: 0 }),
    (Point { x: 0, y: 9 }, Point { x: 2, y: 9 }),
    (Point { x: 3, y: 4 }, Point { x: 1, y: 4 }),
    (Point { x: 0, y: 0 }, Point { x: 8, y: 8 }),
    (Point { x: 5, y: 5 }, Point { x: 8, y: 2 }),
];
