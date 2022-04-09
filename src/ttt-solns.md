# Solutions

## Task 0

If you haven't installed Rust/cargo already, head to <https://rustup.rs>

```bash
cargo new tic-tac-toe
cd tic-tac-toe
code .
```

Exchange `code` for your editor of choice.

## Task 1.1

You're looking for an enum:

```rust,noplayground
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Player {
    X,
    O,
}
```

Note that I've [`derive`](https://doc.rust-lang.org/book/ch05-02-example-structs.html?highlight=derive#adding-useful-functionality-with-derived-traits)d some traits on this type, which will come in handy later:

- `Eq` and `PartialEq` are used for evaluating the equality of value of that type
- `Copy` and `Clone` tells the compiler that it is free to copy the type all over the place
  - This means we don't have to worry about move semantics for now. That's a lesson for next time.

## Task 1.2

There's a few different ways to go about this. You could use either a 2-d array, or 1-d array, whichever you prefer. The way I opted to use a 2-d array of `Option<Player>`, which represents either `Some(Player)`, or `None`, when the square is empty. I then wrapped that array in a struct, which also holds who's turn it currently is, and if there is currently a winner. This allows the struct to represent more the state of the entire game than just the board, but this is entirely up to you.

```rust,noplayground
struct Board {
    grid: [[Option<Player>; 3]; 3],
    current_turn: Player,
    winner: Option<Player>,
}
```

You could also opt to not hold the current player or winner in the struct, in which case a [type alias](https://doc.rust-lang.org/book/ch19-04-advanced-types.html?highlight=type%20alias#creating-type-synonyms-with-type-aliases) would make more sense.

```rust,noplayground
type Board = [[Option<Player>; 3]; 3]
```

## Task 1.3

The `let mut` expression creates a new, mutable, instance of our `Board` struct from above. We then iterate through each square, adding in some decoration too.

```rust,noplayground
let mut board = Board {
        grid: [[None, None, None], [None, None, None], [None, None, None]],
        current_turn: Player::X,
        winner: None,
    };

println!("-------------");
for row in board.grid {
    for square in row {
        print!("|");
        match square {
            Some(p) => print!(" {} ", p),
            None => print!("   "),
        }
    }
    println!("|");
    println!("-------------");
}
```

will print something like:

```
-------------
| X | O | X |
-------------
| O | X | O |
-------------
| X | O | X |
-------------
```

Note how we're using our `Player` enum within the `print!()` macro. This is because I manually implemented the `Display` trait on it:

```rust,noplayground
impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            match self {
                Player::X => "X",
                Player::O => "O",
            }
        )
    }
}
```

This may look a little scary for now, because it is. Pattern matching on `Option<Player>` in the loop just as good:

```rust,noplayground
println!("-------------");
for row in board.grid {
    for square in row {
        print!("|");
        match square {
            Some(Player::X) => print!(" {X} "),
            Some(Player::O) => print!(" {O} "),
            None => print!("   "),
        }
    }
    println!("|");
    println!("-------------");
}
```

You could also implement the `Display` trait on the entire `Board` if you wished, moving the above code to the `fmt` function, similar to `Player`.

## Task 2.1

You'll need `std::io`, which is a module from Rust's standard library. Modules are imported in Rust using the `use` keyword, so something like:

```rust,noplayground
use std::io::stdin;
use std::io::stdout;
```

will import those modules. You can also combine them if you want:

```rust,noplayground
use std::io::{stdin, stdout};
```

You could use a `while` loop, with a condition checking for winners, or a `loop` with a `break`. I opted for the latter approach.

```rust,noplayground
fn main() {
    let mut board = Board {
        grid: [[None, None, None], [None, None, None], [None, None, None]],
        current_turn: Player::X,
        winner: None,
    };

    loop {
        //prompt for user input
        print!("Player {}, enter a square>>", board.current_turn);
        //flush stdout because stdout is weird
        stdout().flush();

        //create the buffer our input will be copied into
        let mut turn = String::new();
        //read input into that buffer,
        stdin().read_line(&mut turn);

        //print the board
        println!("-------------");
        for row in board.grid {
            for square in row {
                print!("|");
                match square {
                    Some(Player::X) => print!(" {X} "),
                    Some(Player::O) => print!(" {O} "),
                    None => print!("   "),
                }
            }
            println!("|");
            println!("-------------");
        }
    }
}
```

The reason we have to flush `stdout` manually is because it is usually flushed when there is a newline character or the runtime's buffer fills up, but neither of these things happen.

Our board printing code is also included there at the bottom of the loop, but it doesn't do anything yet really, as using the input comes next. Our game is starting to come together!

## Task 2.2

The following snippet validates the input is a number, in the range of the board, in a blank square. It then adds the turn to the board if so.

```rust,noplayground
let guess: Result<usize, _> = turn.trim().parse();

if guess.is_err() {
    continue;
}
let square = guess.unwrap() - 1;
let row = square / 3;
let column = square % 3;

if square > 8 || board.grid[row][column].is_some() {
    continue;
}

//add the turn to the board
board.grid[row][column] = Some(board.current_turn);
```

[`parse()`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.parse) is a funny little function, as it is generic over any type that a string can be turned into, hence why we have to use the affectionately-named turbofish syntax to specify what type we want to parse our string into. It also returns a `Result<T,E>`, which is like an upgraded version of `Option<T>`, expressing that the function returns either our result `T`, or some type expressing an error `E`. We check that the `Result` is not an error, and then unwrap the guess from it.

We use one conditional expression to check if our parse function failed using `is_err()`, then we can `unwrap()` our number from the `Result`. Another condition is used to check the square is not out of range, or already taken. We jump back to the top of the loop in any of these cases.

## Task 2.3

We can just add a simple `match` expression at the bottom of our loop to switch turns.

```rust,noplayground
board.current_turn = match board.current_turn {
    Player::O => Player::X,
    Player::X => Player::O,
}
```

Depending upon how your loop works you might need to put this somewhere else to handle the control flow differently.

Our main function now looks like this. I added a little help text at the top to print at the start of the game, too!

```rust,noplayground
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
        turn.clear();

        if guess.is_err() {
            continue;
        }
        let square = guess.unwrap() - 1;
        if square > 8 || board.grid[square / 3][square % 3].is_some() {
            continue;
        }

        //print the board
        board.grid[square / 3][square % 3] = Some(board.current_turn);
    }
}
```

## Task 3

This bit is a little more complicated. `Board.grid` is an array of `Option<Player>`, so we need to check that if each tile in the row is equal, and also that they are not all `None` values (done by the `is_some()` method). We check each row, each column, and also the two diagonals. If any of these checks end up storing a winner in `board.winner`, then the `match` at the bottom catches this and ends the game.

```rust,noplayground
//check if we have any winners
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
```

The `Eq` and `PartialEq` derives from earlier allow us to use the `==` operator to compare instances of our `Player` type. More info about those can be found [in The Book](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html#partialeq-and-eq-for-equality-comparisons)

## The Final Product

The full code can be found on github: <https://github.com/Joeyh021/rs118-tic-tac-toe>
