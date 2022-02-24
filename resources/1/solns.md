# Solutions

## Task 0

If you haven't installed rust/cargo already, head to <https://rustup.rs>

```bash
cargo new tic-tac-toe
cd tic-tac-toe
code .
```

Exchange `code` for your editor of choice.

## Task 1.1

You're looking for an enum:

```rust
{{#include ../../tic-tac-toe/src/main.rs:5:9}}
```

Note that I've [`derive`](https://doc.rust-lang.org/book/ch05-02-example-structs.html?highlight=derive#adding-useful-functionality-with-derived-traits)d some traits on this type, which will come in handy later:

- `Eq` and `PartialEq` are used for evaluating the equality of value of that type
- `Copy` and `Clone` tells the compiler that it is free to copy the type all over the place
  - This means we don't have to worry about move semantics for now. That's a lesson for next time.

## Task 1.2

There's a few different ways to go about this. You could use either a 2-d array, or 1-d array, whichever you prefer. The way I opted to do this to use an array of `Option<Player>`, which represents either `Some(Player)`, or `None`, when the square is empty. I then wrapped that array in a struct, which also holds who's turn it currently is, and if there is currently a winner. This allows the struct to represent more the state of the entire game than just the board, but this is entirely up to you.

```rust
{{#include ../../tic-tac-toe/src/main.rs:22:26}}
```

You could also opt to not hold the current player or winner in the struct, in which case a [type alias](https://doc.rust-lang.org/book/ch19-04-advanced-types.html?highlight=type%20alias#creating-type-synonyms-with-type-aliases) would make more sense.

```rust
type Board = [[Option<Player>; 3]; 3]
```

## Task 1.3

Using the 2-d array approach from above and iterating through each square, adding in some decoration too:

```rust
{{#include ../../tic-tac-toe/src/main.rs:67:78}}
```

will print:

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

```rust
{{#include ../../tic-tac-toe/src/main.rs:10:21}}
```

This may look a little scary for now, because it is. Pattern matching on the `Player` enum too is just as good:

```rust
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

## Task 2.1

You'll need `std::io`, which is a module from Rust's standard library. Modules are imported in rust using the `use` keyword, so something like:

```rust
use std::io::stdin;
use std::io::stdout;
```

will import those modules. You can also combine them if you want:

```rust
use std::io::{stdin,stdout};
```

You could use a `while` loop, with a condition checking for winners, or a `loop` with a `break`. I opted for the latter approach.

```rust
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

        //read input into the buffer
        stdin().read_line(&mut turn).expect("Failed to read line");

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

Our board printing code is also included there at the bottom of the loop, but it doesn't do anything yet really, as using the input comes next. I also created a mutable instance of our struct there at the top. Our game is starting to come together!

## Task 2.2

The following snippet validates the input is a number, in the range of the board, in a blank square:

```rust
{{#include ../../tic-tac-toe/src/main.rs:53:64}}
```

[`parse()`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.parse) is a funny little function, as it is generic over any type that a string can be turned into, hence why we have to use the affectionately-named turbofish syntax to specify what type we want to parse our string into. It also returns a `Result<T,E>`, which is like an upgraded version of `Option<T>`.

We use one conditional statement to check if our parse function failed using `is_err()`, then we can `unwrap()` our number from the `Result`. Another condition is used to check the square is not out of range, or already taken. We jump back to the top of the loop in any of these cases.

A simple `if`/`else` is added at the bottom to swap player each loop iteration.

Here it is in the context of the our `main()` function, put just before we print the board:

```rust
fn main() {
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
        {
            let board = &board;
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
        };

        if board.current_turn == Player::X {
            board.current_turn = Player::O;
        } else {
            board.current_turn = Player::X;
        }
    }
}
```

## Task 3

This bit is a little more complicated. `Board.grid` is an array of `Option<Player>`, so we need to check that if each tile in the row is equal, and also that they are not all `None` values (done by the `is_some()` method). We check each row, each column, and also the two diagonals. If any of these checks end up storing a winner in `board.winner`, then the `match` at the bottom catches this and ends the game.

```rust
{{#include ../../tic-tac-toe/src/main.rs:79:119}}
```

The `Eq` and `PartialEq` derives from earlier allow us to use the `==` operator to compare instances of our `Player` type. More info about those can be found [in the book](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html#partialeq-and-eq-for-equality-comparisons)

## The Final Product

The full code can be found on github GIT REPO LINK

Here's the full final code listing, if you're interested:

{{#playground ../../tic-tac-toe/src/main.rs}}
