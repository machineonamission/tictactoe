use std::io;

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

fn check_for_win(board: &[BoardSpace; 9]) -> BoardSpace {
    for win_case in WINS {
        // 3 spaces in a row are identical
        if board[win_case[0]] == board[win_case[1]]
            && board[win_case[0]] == board[win_case[2]]
            // and not empty
            && board[win_case[0]] != BoardSpace::Empty {
            return board[win_case[0]].clone();
        }
    }
    BoardSpace::Empty
}

fn print_board([a, b, c, d, e, f, g, h, i]: &[BoardSpace; 9]) {
    println!(
        "{a} | {b} | {c}\n\
         --+---+--\n\
         {d} | {e} | {f}\n\
         --+---+--\n\
         {g} | {h} | {i}",
    )
}

fn make_move(board: &mut [BoardSpace; 9], player: &BoardSpace, space: usize) -> Result<(), anyhow::Error> {
    if *player == BoardSpace::Empty {
        return Err(anyhow::anyhow!("Attempted to make move for empty player."));
    }
    if board[space] != BoardSpace::Empty {
        return Err(anyhow::anyhow!("Attempted to make move on a non-empty space."));
    }
    board[space] = player.clone();
    Ok(())
}

fn get_player_move(board: &mut [BoardSpace; 9], player: &BoardSpace) {
    loop {
        println!("Input the number of the square to place, ? to see the numbers of the squares, or ! to view the board: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read input");
        let trimput = input.trim();
        match trimput.parse::<usize>() {
            Err(_) => {
                match trimput {
                    "?" => {
                        println!(
                            "0 | 1 | 2\n\
                             --+---+--\n\
                             3 | 4 | 5\n\
                             --+---+--\n\
                             6 | 7 | 8"
                        )
                    }
                    "!" => {
                        print_board(board);
                    }
                    _ => {
                        println!("Invalid input.")
                    }
                }
            }
            Ok(res) => {
                // linter told me 0 is minimum for usize so no need to check
                if res <= 8usize {
                    match make_move(board, player, res) {
                        Ok(_) => {
                            return;
                        }
                        Err(error) => {
                            println!("{}", error);
                        }
                    }
                } else {
                    println!("Valid spaces are 0-8.")
                }
            }
        }
    }
}

fn main() {
    let mut board: [BoardSpace; 9] = Default::default();
    println!("Hello, world!");
    loop {
        for player in [BoardSpace::X, BoardSpace::O] {
            print_board(&board);
            println!("Player {}'s turn!", player);
            get_player_move(&mut board, &player);
            let winner = check_for_win(&board);
            if winner != BoardSpace::Empty {
                print_board(&board);
                println!("Player {} won!", winner);
                return;
            }
        }
    }
}
