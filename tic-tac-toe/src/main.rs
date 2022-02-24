use std::{
    fmt::Display,
    io::{stdin, stdout, Write},
};
#[derive(PartialEq, Eq, Copy, Clone)]
enum Player {
    X,
    O,
}
impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Player::X => "X",
                Player::O => "O",
            }
        )
    }
}
struct Board {
    grid: [[Option<Player>; 3]; 3],
    current_turn: Player,
    winner: Option<Player>,
}

fn main() {
    println!("tic tac toe!");
    println!("Board squares are numbered as follows:");
    println!(
        "------------\n\
        | 1 | 2 | 3 |\n\
        -------------\n\
        | 4 | 5 | 6 |\n\
        -------------\n\
        | 7 | 8 | 9 |\n\
        -------------"
    );

    let mut board = Board {
        grid: [[None, None, None], [None, None, None], [None, None, None]],
        current_turn: Player::X,
        winner: None,
    };
    loop {
        print!("Player {}, enter a square>>", board.current_turn);
        stdout().flush().expect("Could not flush stdout");

        let mut turn = String::new();

        stdin().read_line(&mut turn).expect("Failed to read line");
        let guess: Result<usize, _> = turn.trim().parse();

        if guess.is_err() {
            continue;
        }
        let square = guess.unwrap() - 1;
        if square > 8 || board.grid[square / 3][square % 3].is_some() {
            continue;
        }

        //print the board
        board.grid[square / 3][square % 3] = Some(board.current_turn);

        println!("-------------");
        for row in board.grid {
            for square in row {
                print!("|");
                match square {
                    Some(p) => print!(" {p} "),
                    None => print!("   "),
                }
            }
            println!("|");
            println!("-------------");
        }

        //check if we have any winnders
        //check rows -- easily done
        for row in board.grid {
            if row[0] == row[1] && row[1] == row[2] && row[0].is_some() {
                board.winner = row[0];
            }
        }
        //check columns -- need some indexing for this
        for i in 0..3_usize {
            if board.grid[0][i] == board.grid[1][i]
                && board.grid[1][i] == board.grid[2][i]
                && board.grid[0][i].is_some()
            {
                board.winner = board.grid[0][i];
            }
        }
        //check diagonals
        if board.grid[0][0] == board.grid[1][1]
            && board.grid[1][1] == board.grid[2][2]
            && board.grid[0][0].is_some()
        {
            board.winner = board.grid[0][0];
        }
        if board.grid[0][2] == board.grid[1][1]
            && board.grid[1][1] == board.grid[2][0]
            && board.grid[0][2].is_some()
        {
            board.winner = board.grid[0][2];
        }

        match board.winner {
            Some(Player::X) => {
                print!("X wins");
                break;
            }
            Some(Player::O) => {
                print!("O wins");
                break;
            }
            None => (),
        }

        if board.current_turn == Player::X {
            board.current_turn = Player::O;
        } else {
            board.current_turn = Player::X;
        }
    }
}
