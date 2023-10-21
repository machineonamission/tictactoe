#[derive(PartialEq)]
#[derive(Clone)]
enum BoardSpace {
    EMPTY,
    X,
    O,
}
impl std::fmt::Display for BoardSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            BoardSpace::EMPTY => "",
            BoardSpace::O => "O",
            BoardSpace::X => "X"
        })
    }
}
// could I generate this dynamically? yes. Is it easier and faster to do it manually? also yes
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
            && board[win_case[0]] != BoardSpace::EMPTY {
            return &board[win_case[0]];
        }
    }
    return &BoardSpace::EMPTY;
}

fn main() {
    let board: [BoardSpace; 9];
    println!("Hello, world!");
}
