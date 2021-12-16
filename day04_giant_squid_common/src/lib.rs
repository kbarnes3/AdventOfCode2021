pub struct BingoBoard {
    pub board: [[u32; 5]; 5],
}

pub struct InputData<const N: usize, const B: usize> {
    pub draw_numbers: [u32; N],
    pub boards: [BingoBoard; B],
}

// Substitute boards with
// '<,'>s/^$/        ]},\r        BingoBoard { board: [/ | '<,'>s/ *\(\d\+\) \+\(\d\+\) \+\(\d\+\) \+\(\d\+\) \+\(\d\+\)/            [ \1, \2, \3, \4, \5 ],/
pub const SAMPLE_DATA: InputData<27, 3> = InputData {
    draw_numbers: [7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1],
    boards: [
        BingoBoard { board: [
            [ 22, 13, 17, 11, 0 ],
            [ 8, 2, 23, 4, 24 ],
            [ 21, 9, 14, 16, 7 ],
            [ 6, 10, 3, 18, 5 ],
            [ 1, 12, 20, 15, 19 ],
        ]},
        BingoBoard { board: [
            [ 3, 15, 0, 2, 22 ],
            [ 9, 18, 13, 17, 5 ],
            [ 19, 8, 7, 25, 23 ],
            [ 20, 11, 10, 24, 4 ],
            [ 14, 21, 16, 12, 6 ],
        ]},
        BingoBoard { board: [
            [ 14, 21, 17, 24, 4 ],
            [ 10, 16, 15, 9, 19 ],
            [ 18, 8, 23, 26, 20 ],
            [ 22, 11, 13, 6, 5 ],
            [ 2, 0, 12, 3, 7 ],
        ]},
    ],
};

