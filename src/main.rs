#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Default)]
enum BoardSpace {
    #[default]
    Empty,
    X,
    O,
}

// to string
impl std::fmt::Display for BoardSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardSpace::Empty => " ",
            BoardSpace::O => "O",
            BoardSpace::X => "X"
        }.fmt(f)
    }
}


// could I generate this dynamically? yes. Is it easier and faster to do it manually? also yes
// list of positions in the board that, if theyre all matched, means someone won
static WINS: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6]
];

fn check_for_win(board: &[BoardSpace; 9]) -> &BoardSpace {
    for win_case in WINS {
        // 3 spaces in a row are identical
        if board[win_case[0]] == board[win_case[1]]
            && board[win_case[0]] == board[win_case[2]]
            // and not empty
            && board[win_case[0]] != BoardSpace::Empty {
            return &board[win_case[0]];
        }
    }
    return &BoardSpace::Empty;
}

fn print_board([a, b, c, d, e, f, g, h, i]: &[BoardSpace; 9]) {
    println!(
        "{a}|{b}|{c}\n\
        ------\n\
        {d}|{e}|{f}\n\
        ------\n\
        {g}|{h}|{i}",
    )
}

fn main() {
    let mut board: [BoardSpace; 9] = Default::default();
    println!("Hello, world!");
    print_board(&board);
}
